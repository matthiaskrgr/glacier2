trait Trait {                                                                   
  async fn method<'a>();                                                        
}                                                                               
                                                                                
impl<'a> Trait for Something {                                                  
  async fn method<'a>() { }                                                     
}   
