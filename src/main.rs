mod api;

use api::echo::{
    echo,
};
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
//fn main() {
    let path = Path::new("/root/tmp_project/morpheus/python_app");
    let py_app = fs::read_to_string(path.join("broker.py")).unwrap();
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("Broker")?
            .into();
        let obj = app.call0(py).unwrap().getattr(py, "broke")?.into();
        Ok(obj)
    });

    // let res = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
    //     from_python.unwrap().call0(py)
    // });
    // print!("result: {}", res.unwrap())
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