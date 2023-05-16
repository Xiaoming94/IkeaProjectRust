use ikea_project_rust::warehouse;
use std::collections::HashMap;

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