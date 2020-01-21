use crate::element::Div;

pub trait Component<C>{
    fn render(&self) -> UITreeNodeInstance<C>;
}

pub enum UITreeNodeInstance<S>{
    Instance(Box<ComponentInstance<S>>),
    Element(Div<S>)
}

pub struct ComponentInstance<C>{
    state: C,
    document: UITreeNodeInstance<C>
}

impl<C: Component<C>> ComponentInstance<C>{
    pub fn new(state: C)-> Self{
        let document = state.render();
        ComponentInstance{
            state,
            document
        }
    }
    pub fn event(&mut self){
        
    }
}

//
//
// user code

pub struct TestCounter{
    count: usize,
    sub_item: bool,
}

impl TestCounter{
    fn add(&mut self){
        self.count+=1;
    }
    
}

impl Component<TestCounter> for TestCounter{
    fn render(&self) -> UITreeNodeInstance<Self>{
        let mut div = Div::new();
        div.listener(
            |_, counter: &mut Self|{
                counter.add()
            }
        );
        UITreeNodeInstance::Element(div)
    }
}


pub struct MyUI{
    counter: TestCounter,
}


impl Component<MyUI> for MyUI{
    fn render(&self) -> UITreeNodeInstance<Self>{
        let mut counter_com_instance = ComponentInstance::new(self.counter);
        // div.listener(
        //     |_, counter: &mut Self|{
        //         counter.add()
        //     }
        // );
        UITreeNodeInstance::Instance(Box::new(counter_com_instance))
    }
}