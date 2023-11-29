use serenity::all::User;

pub static DMS: [&str; 2] = [
    "279145098426580992", // Fin
    "205811939861856257", // Flora
];

pub fn is_dm(user: &User) -> bool {
    DMS.contains(&user.id.to_string().as_str())
}