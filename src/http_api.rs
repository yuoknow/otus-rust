use crate::house::devices::device::Device;
use crate::house::devices::smart_socket::SmartSocket;
use crate::house::devices::smart_thermometer::SmartThermometer;
use crate::house::smart_house::SmartHouse;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

pub async fn start_web_server() {
    let socket1 = SmartSocket {
        _description: "socket1".to_string(),
        _is_enabled: false,
        _power: 10.0,
    };
    let thermo = SmartThermometer {
        _current_temperature: 20.0,
    };

    let mut house = SmartHouse::new();
    house.add_room("room1".to_string()).unwrap();
    house
        .add_device(
            "room1".to_string(),
            "socket1".to_string(),
            Device::SmartSocket(socket1),
        )
        .unwrap();

    house.add_room("room2".to_string()).unwrap();
    house
        .add_device(
            "room1".to_string(),
            "thermo".to_string(),
            Device::SmartThermometer(thermo),
        )
        .unwrap();

    let state = AppState {
        data: Arc::new(Mutex::new(house)),
    };

    let app = Router::new()
        .route(
            "/house/rooms",
            get(get_house_rooms)
                .post(create_house_room)
                .delete(remove_house_room),
        )
        .route(
            "/house/rooms/:room/devices",
            get(get_room_devices).post(add_device).delete(remove_device),
        )
        .route("/house/rooms/:room/devices/:device", get(get_room_device))
        .route("/house/report", get(get_report))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<SmartHouse>>,
}

async fn get_house_rooms(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({"rooms": state.data.lock().expect("mutex was poisoned").get_rooms()}))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateRoom {
    name: String,
}

async fn create_house_room(
    State(state): State<AppState>,
    Json(request): Json<CreateRoom>,
) -> (StatusCode, String) {
    return match state
        .data
        .lock()
        .expect("mutex was poisoned")
        .add_room(request.name)
    {
        Ok(_) => (StatusCode::OK, "".to_string()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RemoveRoom {
    name: String,
}

async fn remove_house_room(
    State(state): State<AppState>,
    Json(request): Json<RemoveRoom>,
) -> (StatusCode, String) {
    return match state
        .data
        .lock()
        .expect("mutex was poisoned")
        .remove_room(request.name)
    {
        Ok(_) => (StatusCode::OK, "".to_string()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    };
}

async fn get_room_devices(
    Path(room): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    Json(json!({"devices": state.data.lock().expect("mutex was poisoned").get_devices(&room)}))
}

async fn get_room_device(
    Path((room, device)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    Json(
        json!({"device": state.data.lock().expect("mutex was poisoned").get_device(&room, &device)}),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AddDevice {
    name: String,
    device: Device,
}

async fn add_device(
    Path(room_name): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<AddDevice>,
) -> (StatusCode, String) {
    return match state.data.lock().expect("mutex was poisoned").add_device(
        room_name,
        request.name,
        request.device,
    ) {
        Ok(_) => (StatusCode::OK, "".to_string()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RemoveDevice {
    device: String,
}

async fn remove_device(
    Path(room_name): Path<String>,
    State(state): State<AppState>,
    Json(request): Json<RemoveDevice>,
) -> (StatusCode, String) {
    return match state
        .data
        .lock()
        .expect("mutex was poisoned")
        .remove_device(room_name, request.device)
    {
        Ok(_) => (StatusCode::OK, "".to_string()),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    };
}

async fn get_report(State(state): State<AppState>) -> impl IntoResponse {
    let report = state
        .data
        .lock()
        .expect("mutex was poisoned")
        .create_all_devices_report();
    Json(json!({"report": report}))
}
