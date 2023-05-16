pub fn create_warehouse(name: String) -> Warehouse
{
    Warehouse {m_name: String::from(name)}
}

pub struct Warehouse {
    m_name: String
}