pub mod data_access {
    use mysql::*;
    use mysql::prelude::*;

    pub fn connection() -> PooledConn {
        let url = "mysql://rustuser:roschester@localhost:3306/rustdata";
        let pool = Pool::new(url).expect("wrong");
        let conn = pool.get_conn().expect("wrong"); 
        conn
    }

    pub fn validate_user(mut conn: PooledConn, name: String, password: String) -> bool {
        #[derive(Debug, PartialEq, Eq)]
        struct User {
            username: Option<String>,
        }

        let stmt = conn.prep("select username from users where username=:name and user_password=:password").unwrap();
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
}
