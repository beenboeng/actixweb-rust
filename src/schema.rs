diesel::table! {
    tbl_users (id) {
        id -> Int4,
        #[max_length = 20]
        first_name -> Nullable<Varchar>,
        #[max_length = 20]
        last_name -> Nullable<Varchar>,
        #[max_length = 40]
        user_name -> Nullable<Varchar>,
        #[max_length = 100]
        password -> Nullable<Varchar>,
        #[max_length = 350]
        login_session -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>
    }
}