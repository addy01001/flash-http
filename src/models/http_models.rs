use leptos::{create_rw_signal, RwSignal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpHashMapData {
    pub value: RwSignal<String>,
    pub key: RwSignal<String>,
}

impl HttpHashMapData {
    pub fn new()->HttpHashMapData {
        HttpHashMapData{ 
            value: create_rw_signal(String::new()), 
            key: create_rw_signal(String::new())
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpFormData {
    pub value: RwSignal<String>,
    pub key: RwSignal<String>,
    pub val_type: RwSignal<String>
}

impl HttpFormData {
    pub fn new()->HttpFormData {
        HttpFormData{ 
            value: create_rw_signal(String::new()), 
            key: create_rw_signal(String::new()),
            val_type: create_rw_signal(String::new())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpResponse {
    pub headers: String,
    pub body: String,
    pub code: i32,
    pub timing: f64,
    pub err: String
}

impl HttpResponse {
    pub fn new()->HttpResponse {
        HttpResponse { headers: String::new(), body: String::new(), code: 0, timing: 0.00, err: String::new() }
    }
}
