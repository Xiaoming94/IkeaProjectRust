use ikea_project_rust::warehouse;
use std::collections::HashMap;
use ikea_project_rust::customer;

#[test]
fn create_warehouse()
{
   let name: String = String::from("warehouse name");
   warehouse::create_warehouse(name); 
}

#[test]
fn receive_shipment()
{
   let shipment = HashMap::from(
      [
         (String::from("stol"),1),
         (String::from("table"),2),
      ]
   );

   let m_warehouse = warehouse::create_warehouse(String::from("my warehouse"));
   warehouse::receive_shipment(m_warehouse, shipment);
}

#[test]
fn create_customer() {
    let name: String = String::from("Xiaoming");
    customer::create_customer(name);
}

#[test]
fn customer_send_request() {
    let m_customer = customer::create_customer(String::from("Xiaoming"));
    let m_warehouse = warehouse::create_warehouse(String::from("my warehouse"));
    customer::customer_send_request(m_customer, m_warehouse);
}
