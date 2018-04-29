
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;
extern crate xml;

use std::io::Read;
use xml::reader::EventReader;
use yaserde::YaDeserialize;

macro_rules! convert_and_validate {
  ($content:expr, $struct:tt, $model:expr) => {
    let mut parser = EventReader::from_str($content);

    let loaded = $struct::derive_deserialize(&mut parser, None);
    assert_eq!(loaded, Ok($model));
  }
}

#[test]
fn de_basic() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    item: String
  }

  let content = "<base><item>something</item></base>";
  convert_and_validate!(content, XmlStruct, XmlStruct{
    item: "something".to_string()
  });
}

#[test]
fn de_list_of_items() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    items: Vec<String>
  }

  let content = "<base><items>something1</items><items>something2</items></base>";
  convert_and_validate!(content, XmlStruct, XmlStruct{
    items: vec![
      "something1".to_string(),
      "something2".to_string()
    ]
  });

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStructOfStruct {
    items: Vec<SubStruct>
  }


  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="items")]
  pub struct SubStruct {
    field: String
  }

  let content = "<base><items><field>something1</field></items><items><field>something2</field></items></base>";
  convert_and_validate!(content, XmlStructOfStruct, XmlStructOfStruct{
    items: vec![
      SubStruct{
        field: "something1".to_string()
      },
      SubStruct{
        field: "something2".to_string()
      }
    ]
  });
}

#[test]
fn de_attributes() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    item: String,
    sub: SubStruct
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="sub")]
  pub struct SubStruct {
    #[yaserde(attribute)]
    subitem: String
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct{
        subitem: "".to_string()
      }
    }
  }

  let content = "<base item=\"something\"><sub subitem=\"sub-something\"></sub></base>";
  convert_and_validate!(content, XmlStruct, XmlStruct{
    item: "something".to_string(),
    sub: SubStruct{
      subitem: "sub-something".to_string()
    }
  });
}

#[test]
fn de_rename() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename="Item")]
    item: String,
    #[yaserde(rename="sub")]
    sub_struct: SubStruct
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename="sub_item")]
    subitem: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct{
        subitem: "".to_string()
      }
    }
  }

  let content = "<base Item=\"something\"><sub sub_item=\"sub_something\"></sub></base>";
  convert_and_validate!(content, XmlStruct, XmlStruct{
    item: "something".to_string(),
    sub_struct: SubStruct{
      subitem: "sub_something".to_string()
    }
  });
}

#[test]
fn de_text_content_with_attributes() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename="Item")]
    item: String,
    #[yaserde(rename="sub")]
    sub_struct: SubStruct
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename="sub_item")]
    subitem: String,
    #[yaserde(text)]
    text: String
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct{
        subitem: "".to_string(),
        text: "".to_string(),
      }
    }
  }

  let content = "<base Item=\"something\"><sub sub_item=\"sub_something\">text_content</sub></base>";
  convert_and_validate!(content, XmlStruct, XmlStruct{
    item: "something".to_string(),
    sub_struct: SubStruct{
      subitem: "sub_something".to_string(),
      text: "text_content".to_string()
    }
  });
}

#[test]
fn de_enum() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    background: Color
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="color")]
  pub enum Color {
    White,
    Black,
  }

  impl Default for Color {
    fn default() -> Color {
      Color::White
    }
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  pub struct RGBColor {
    red: String,
    green: String,
    blue: String,
  }

  impl Default for RGBColor {
    fn default() -> RGBColor {
      RGBColor{
        red: "0".to_string(),
        green: "0".to_string(),
        blue: "0".to_string(),
      }
    }
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  pub enum Alpha {
    Transparent,
    Opaque,
  }

  impl Default for Alpha {
    fn default() -> Alpha {
      Alpha::Transparent
    }
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><background>Black</background></base>";
  convert_and_validate!(content, XmlStruct, XmlStruct{
    background: Color::Black
  });
}
