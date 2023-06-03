use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use glam::{vec3a, Vec3A};
use serde::{
    de::{self, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};

#[derive(Clone, Copy)]
pub struct Vec3(Vec3A);

impl Serialize for Vec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Vec3", 3)?;
        s.serialize_field("x", &self.0.x)?;
        s.serialize_field("y", &self.0.y)?;
        s.serialize_field("z", &self.0.z)?;
        s.end()
    }
}

const VEC3_FIELDS: [&'static str; 3] = ["x", "y", "z"];

enum Vec3Field {
    X,
    Y,
    Z,
}

struct Vec3FieldVisitor;

impl<'de> Visitor<'de> for Vec3FieldVisitor {
    type Value = Vec3Field;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("'x', 'y' or 'z'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "x" => Ok(Vec3Field::X),
            "y" => Ok(Vec3Field::Y),
            "z" => Ok(Vec3Field::Z),
            _ => Err(de::Error::unknown_field(v, &VEC3_FIELDS)),
        }
    }
}

impl<'de> Deserialize<'de> for Vec3Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(Vec3FieldVisitor)
    }
}

struct Vec3Visitor;

impl<'de> Visitor<'de> for Vec3Visitor {
    type Value = Vec3;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Vec3")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;
        let mut z = None;
        while let Some(k) = map.next_key()? {
            match k {
                Vec3Field::X => {
                    if x.is_some() {
                        return Err(de::Error::duplicate_field("x"));
                    } else {
                        x = Some(map.next_value()?);
                    }
                }
                Vec3Field::Y => {
                    if y.is_some() {
                        return Err(de::Error::duplicate_field("y"));
                    } else {
                        y = Some(map.next_value()?);
                    }
                }
                Vec3Field::Z => {
                    if z.is_some() {
                        return Err(de::Error::duplicate_field("z"));
                    } else {
                        z = Some(map.next_value()?);
                    }
                }
            }
        }
        let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
        let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
        let z = z.ok_or_else(|| de::Error::missing_field("z"))?;
        Ok(vec3(x, y, z))
    }
}

impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = Vec3Visitor {};
        deserializer.deserialize_struct("Vec3", &VEC3_FIELDS, v)
    }
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3(vec3a(x, y, z))
}

impl Vec3 {
    pub fn normalize(self) -> Self {
        Self(self.0.normalize())
    }

    pub fn dot(self, s: Self) -> f32 {
        self.0.dot(s.0)
    }

    pub fn length_squared(self) -> f32 {
        self.0.length_squared()
    }

    pub fn cross(self, s: Self) -> Self {
        Self(self.0.cross(s.0))
    }

    pub fn x(self) -> f32 {
        self.0.x
    }

    pub fn y(self) -> f32 {
        self.0.y
    }

    pub fn z(self) -> f32 {
        self.0.z
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0.mul(rhs))
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.mul(rhs.0))
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0.div(rhs))
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0)
    }
}
