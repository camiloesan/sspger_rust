pub mod data_access {
    use mysql::*;
    use mysql::prelude::*;

    pub fn connection() -> PooledConn {
        let url = "mysql://spglisoft_user:apple@localhost:3306/SPGLISOFT";
        let pool = Pool::new(url).expect("wrong");
        let conn = pool.get_conn().expect("wrong"); 
        conn
    }

    pub fn validate_user(mut conn: PooledConn, name: String, password: String) -> bool {
        #[derive(Debug, PartialEq, Eq)]
        struct User {
            username: Option<String>,
        }

        let stmt = conn.prep("select nombre from desarrollador where matricula=:name and contrasena=:password").unwrap();
        let mut exists: bool = false;

        conn.exec_iter(&stmt, params! { "name" => &name, "password" => &password })
            .unwrap()
            .for_each(|row| {
                let r: String = from_row(row.unwrap());
                println!("{}", r);
                exists = true; 
            });
        exists
    }

    pub fn get_all_developers() -> Vec<(String, String, String)> {
        let query = "SELECT nombre, apellido_paterno, apellido_materno FROM desarrollador";
        let result: Vec<(String, String, String)> = connection().exec_iter(&query, ())
            .unwrap()
            .map(|row| {
                let nombre: String = row.as_ref().expect("err").get(0).unwrap();
                let apellido_paterno: String = row.as_ref().expect("err").get(1).unwrap();
                let apellido_materno: String = row.as_ref().expect("err").get(2).unwrap();
                (nombre, apellido_paterno, apellido_materno)
            })
            .collect();

        result
    }
}