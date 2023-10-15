use crate::{users::UserCollection, user::User};
mod users;
mod user;

fn main() {
    print!("Interators are widely used in the standart library");

    let array = &[1, 2, 3];
    let interator = array.iter();

    interator.for_each(|e| print!("{}", e));

    println!("Let's test our own iterator");

    let mut users = UserCollection::new();
    let user_1 = User{name: "Antonio".into(), age: 20, last_name:"Costa".into()};
    let user_2 = User{name: "Lucas".into(), age: 20, last_name:"Costa".into()};
    let user_3 = User{name: "Marcos".into(), age: 20, last_name:"Costa".into()};
    //let user_4 = User{name: "Samuel".into(), age: 20, last_name:"Costa".into()};
    users.add(user_1);
    users.add(user_2);
    users.add(user_3);
    //users.add(user_4);

    let mut iterator = users.iter();

    println!("1nd element: {:?}", iterator.next());
    println!("2nd element: {:?}", iterator.next());
    println!("3nd element: {:?}", iterator.next());
    println!("4nd element: {:?}", iterator.next());

    let user_4 = iterator.next();

    match user_4 {
        Some(u) => println!("{}",u.name),
        None => println!("Not")
    }

    print!("All elements in user collection");
    users.iter().for_each(|e| print!("{:?} ", e));
    println!();
}

