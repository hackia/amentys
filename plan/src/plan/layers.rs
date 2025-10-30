use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Error};
use uuid::Uuid;

#[doc = "The root layer name is always the first layer in the list of layers. It is always present and cannot be removed."]
pub const ROOT_LAYER_NAME: &str = "root";

///
/// # A layer.
///
/// A layer is composed of:
/// - `id` - The layer id.
/// - `name` - The layer name.
///
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Layer {
    id: Uuid,
    name: String,
}
/// # A list of `Layer`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Layers {
    layers: Vec<Layer>,
}

impl Layer {
    #[must_use]
    #[doc = "Creates a new layer."]
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
        }
    }

    #[doc = "Return the layer id."]
    #[must_use]
    pub const fn get_id(&self) -> Uuid {
        self.id
    }
    #[doc = "Returns the layer name."]
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Layers {
    #[must_use]
    pub fn new() -> Self {
        Self {
            layers: Vec::from([Layer::new(ROOT_LAYER_NAME)]),
        }
    }
    ///
    /// # Save a plan to a file
    ///
    /// # Errors
    /// if the file path exists
    ///
    pub fn save(&self, path: &str) -> Result<(), Error> {
        let fichier = File::create_new(path)?;
        let writer = BufWriter::new(fichier);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
    ///
    /// load a plan from a file
    ///
    /// # Errors
    /// if the file path doesn't exist
    ///
    pub fn load(path: &str) -> Result<Self, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let plan: Self = serde_json::from_reader(reader)?;
        Ok(plan)
    }

    #[doc = "Returns the number of layers."]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.layers.len()
    }
    #[doc = "Returns true if the plan is empty."]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
    #[doc = "Returns the first layer."]
    #[must_use]
    pub fn first(&self) -> Option<&Layer> {
        self.layers.first()
    }
    #[doc = "Returns the last layer."]
    #[must_use]
    pub fn last(&self) -> Option<&Layer> {
        self.layers.last()
    }
    #[doc = "Add a layer."]
    pub fn add(&mut self, layer: Layer) -> &mut Self {
        self.layers.push(layer);
        self
    }
    #[doc = "Update a layer."]
    pub fn update(&mut self, name: &str, data: &Layer) -> &mut Self {
        if name == ROOT_LAYER_NAME {
            return self;
        }
        if let Some(index) = self.layers.iter().position(|l| l.name == name) {
            self.layers[index].name = data.name.to_string();
        }
        self
    }
    #[doc = "Remove a layer."]
    pub fn remove(&mut self, name: &str) -> &mut Self {
        if name == ROOT_LAYER_NAME {
            return self;
        }
        if let Some(index) = self.layers.iter().position(|l| l.name == name) {
            self.layers.remove(index);
        }
        self
    }
    #[doc = "Returns a layer by name."]
    #[must_use]
    pub fn get(&mut self, name: &str) -> Option<&Layer> {
        self.layers.iter().find(|l| l.name == name)
    }
    #[doc = "Returns the layers."]
    pub const fn get_layers(&mut self) -> &Vec<Layer> {
        &self.layers
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::layers::{Layer, Layers, ROOT_LAYER_NAME};
    use std::fs::remove_file;

    #[test]
    pub fn it_adds_and_removes_layers() {
        let mut x = Layers::new();
        let one = Layer::new("one");
        let two = Layer::new("two");
        let three = Layer::new("three");
        let four = Layer::new("four");
        let five = Layer::new("five");
        assert_eq!(x.layers.len(), 1);
        x.add(one).add(two).add(three).add(four).add(five);
        assert_eq!(x.layers.len(), 6);
    }
    #[test]
    pub fn it_updates_impossible() {
        let mut x = Layers::new();
        let one = Layer::new("one");
        let two = Layer::new("two");
        let three = Layer::new("three");
        let four = Layer::new("four");
        let five = Layer::new("five");
        assert_eq!(x.layers.len(), 1);
        x.add(one);
        x.add(two);
        x.add(three);
        x.add(four);
        x.add(five);
        assert_eq!(x.layers.len(), 6);
        x.update(ROOT_LAYER_NAME, &Layer::new("one_updated"));
        assert_eq!(x.layers.len(), 6);
        x.update("one_updated", &Layer::new("one_updated"));
        assert_eq!(x.layers[0].name, ROOT_LAYER_NAME);
    }
    #[test]
    pub fn it_updates_layers() {
        let mut x = Layers::new();
        let one = Layer::new("one");
        let two = Layer::new("two");
        let three = Layer::new("three");
        let four = Layer::new("four");
        let five = Layer::new("five");
        assert_eq!(x.layers.len(), 1);
        x.add(one);
        x.add(two);
        x.add(three);
        x.add(four);
        x.add(five);
        assert_eq!(x.layers.len(), 6);
        x.update("one", &Layer::new("one_updated"));
        x.update("two", &Layer::new("two_updated"));
        assert_eq!(x.layers[1].name, "one_updated");
        assert_eq!(x.layers[2].name, "two_updated");
        assert_eq!(x.layers[3].name, "three");
        assert_eq!(x.layers[4].name, "four");
        assert_eq!(x.layers[5].name, "five");
    }

    #[test]
    pub fn clone() {
        let mut x = Layers::new();
        let one = Layer::new("one");
        let two = Layer::new("two");
        let three = Layer::new("three");
        let four = Layer::new("four");
        x.add(one);
        x.add(two);
        x.add(three);
        x.add(four);
        assert_eq!(x.layers.len(), 5);
        let mut y = x.clone();
        assert_eq!(x.layers.len(), y.layers.len());
        y.remove("four").remove("three").remove("two").remove("one");
        assert_ne!(x.layers.len(), y.layers.len());
        assert_eq!(y.layers.len(), 1);
        assert_eq!(x.layers.len(), 5);
    }

    #[test]
    pub fn it_save_and_load() {
        let mut x = Layers::new();
        let one = Layer::new("one");
        let two = Layer::new("two");
        let three = Layer::new("three");
        let four = Layer::new("four");
        let five = Layer::new("five");
        x.add(one).add(two).add(three).add(four).add(five);
        assert_eq!(x.layers.len(), 6);
        assert!(x.save("test.json").is_ok());
        let y = Layers::load("test.json");
        assert!(y.is_ok());
        let y = y.unwrap();
        assert_eq!(x.layers.len(), y.layers.len());
        assert!(remove_file("test.json").is_ok());
    }
}
