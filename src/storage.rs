use std::collections::HashMap;

use types::{Element, GreaterElement, Object};

#[derive(Debug, Default)]
pub struct Storage {
    next_object_id: usize,
    next_element_id: usize,
    next_greater_element_id: usize,
    objects: HashMap<ObjectId, Box<Object>>,
    elements: HashMap<ElementId, Box<Element>>,
    greater_elements: HashMap<GreaterElementId, Box<GreaterElement>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage::default()
    }

    pub fn insert_object<T>(&mut self, object: T) -> ObjectId
    where
        T: Object + 'static,
    {
        let key = ObjectId(self.next_object_id);
        self.next_object_id += 1;
        self.objects.insert(key, Box::new(object));
        key
    }
    pub fn insert_element<T>(&mut self, element: T) -> ElementId
    where
        T: Element + 'static,
    {
        let key = ElementId(self.next_element_id);
        self.next_element_id += 1;
        self.elements.insert(key, Box::new(element));
        key
    }
    pub fn insert_greater_element<T>(&mut self, greater_element: T) -> GreaterElementId
    where
        T: GreaterElement + 'static,
    {
        let key = GreaterElementId(self.next_greater_element_id);
        self.next_greater_element_id += 1;
        self.greater_elements.insert(key, Box::new(greater_element));
        key
    }

    pub fn get_object<T>(&self, id: ObjectId) -> Option<&T>
    where
        T: Object,
    {
        self.objects.get(&id).and_then(|x| x.downcast_ref::<T>())
    }
    pub fn get_element<T>(&self, id: ElementId) -> Option<&T>
    where
        T: Element,
    {
        self.elements.get(&id).and_then(|x| x.downcast_ref::<T>())
    }
    pub fn get_greater_element<T>(&self, id: GreaterElementId) -> Option<&T>
    where
        T: GreaterElement,
    {
        self.greater_elements
            .get(&id)
            .and_then(|x| x.downcast_ref::<T>())
    }

    pub fn remove_object<T>(&mut self, id: ObjectId) -> Option<T>
    where
        T: Object,
    {
        // check if we have an object with id and it is of the required type
        if let None = self.objects.get(&id).and_then(|x| x.downcast_ref::<T>()) {
            return None;
        }

        // actually remove and return the object
        self.objects
            .remove(&id)
            .and_then(|x| x.downcast::<T>().ok())
            .map(|x| *x)
    }
    pub fn remove_element<T>(&mut self, id: ElementId) -> Option<T>
    where
        T: Element,
    {
        // check if we have an object with id and it is of the required type
        if let None = self.elements.get(&id).and_then(|x| x.downcast_ref::<T>()) {
            return None;
        }

        // actually remove and return the object
        self.elements
            .remove(&id)
            .and_then(|x| x.downcast::<T>().ok())
            .map(|x| *x)
    }
    pub fn remove_greater_element<T>(&mut self, id: GreaterElementId) -> Option<T>
    where
        T: GreaterElement,
    {
        // check if we have an object with id and it is of the required type
        if let None = self
            .greater_elements
            .get(&id)
            .and_then(|x| x.downcast_ref::<T>())
        {
            return None;
        }

        // actually remove and return the object
        self.greater_elements
            .remove(&id)
            .and_then(|x| x.downcast::<T>().ok())
            .map(|x| *x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GreaterElementId(usize);
