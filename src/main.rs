mod api;

use api::echo::{
    echo,
};
use std::env;
use log::{info, error, debug};
use log4rs;
use actix_web::{App, HttpServer, web};
use std::fs;
use std::io::Result;
use std::path::Path;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList, PyTuple};
use serde::Deserialize;


fn test() -> PyResult<String>{
    Python::with_gil(|py: Python| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        let str: String = format!("Hello {}, I'm Python {}", user, version);
        Ok(str)
    })
}

// fn test2(input: &str) -> PyResult<String> {
//     let args = PyTuple::new(py, &[input]);
//     app.call1(py, args);

//     Ok(format!("py: {}", from_python?))
// }

// #[derive(Debug, Deserialize)]
// struct User {
//    id: i32,
//    name: String,
// }


#[actix_web::main]
 async fn main() -> Result<()> {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();
    info!("Greetings, Morpheus.");

    let path = Path::new("python_app");
    let py_app = fs::read_to_string(path.join("broker.py")).unwrap();
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        debug!("in lambda...");
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;

        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("Model")?
            .into();
        
        let path_holder = env::current_dir().unwrap().join("python_app");
        let app_path = path_holder.to_str().unwrap();
        let kwargs = [("app_path", app_path)].into_py_dict(py);
        let obj = app.call(py, (), Some(kwargs)).unwrap().getattr(py, "predict")?.into();
        Ok(obj)
    });

    let e = web::Data::new(from_python.unwrap());
    println!("Starting server at http://127.0.0.1:8000");
    HttpServer::new(move|| 
        {
            App::new().app_data(e.clone()).service(echo)
        }
        )
        .workers(1)
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
