use std::collections::HashMap;

pub type ShipmentType = HashMap <String, u32>;
type ShipmentRequests = HashMap <String, u32>;

pub fn create_warehouse(name: String) -> Warehouse
{
    Warehouse {m_name: name, m_shipment: HashMap::new(), m_requests: HashMap::new()}
}

pub fn receive_shipment(inWarehouse: Warehouse ,shipment: ShipmentType) -> ()
{

}

pub struct Warehouse {
    m_name: String,
    m_shipment: ShipmentType,
    m_requests: ShipmentRequests,
}
