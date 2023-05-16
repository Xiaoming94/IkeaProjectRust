pub fn create_warehouse(name: String)
{
    Warehouse {name: String::from(name)};
}

struct Warehouse {
    name: String
}