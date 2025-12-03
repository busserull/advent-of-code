mod entrance_password;

fn main() {
    let password = entrance_password::get_entrance_password("safe_input");
    println!("The entrance password is '{}'", password);
}
