struct User {
    user : String,
    email : String,
    active : bool,
    sign_in_count : u64,
}

fn main() {
    let mut user1 = User {
        user : String::from("Dirck"),
        email : String::from("xxx@qq.com"),
        active : true,
        sign_in_count : 1,
    };

    user1.email = String::from("xxx@gmai.com");

    println!("user1:{} email:{} active:{} cnt:{}", user1.user, user1.email, user1.active, user1.sign_in_count);

    let user2 = User {
        email : String::from("xxx@fox.com"),
        ..user1
    };

    // let user2 = build_user("hahaha ".to_string(), "ppp".to_string());

    println!("user2:{} email:{} active:{} cnt:{}", user2.user, user2.email, user2.active, user2.sign_in_count);


}

fn build_user(email_ : String, username_ : String) -> User {
    User {
        email : email_,
        user: username_,
        active : true,
        sign_in_count : 2,
    }
}


