use ikea_project_rust::warehouse;

#[test]
fn create_warehouse()
{
   let name: String = String::from("warehouse name");
   warehouse::create_warehouse(name); 
}