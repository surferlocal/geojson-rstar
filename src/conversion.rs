// Copyright 2015 The GeoRust Developers
// Copyright 2019 Boyd Johnson
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use geo::{Coord, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon};
use geojson::{Geometry, LineStringType, PointType, PolygonType, Value};
use num_traits::Float;
use std::fmt::Debug;
use std::iter::FromIterator;

pub fn create_point_type<T>(point: &Point<T>) -> PointType
where
    T: Float + Debug,
{
    let x: f64 = point.x().to_f64().unwrap();
    let y: f64 = point.y().to_f64().unwrap();

    vec![x, y]
}

pub fn create_line_string_type<T>(line_string: &LineString<T>) -> LineStringType
where
    T: Float + Debug,
{
    line_string
        .points()
        .map(|point| create_point_type(&point))
        .collect()
}

pub fn create_multi_line_string_type<T>(
    multi_line_string: &MultiLineString<T>,
) -> Vec<LineStringType>
where
    T: Float + Debug,
{
    multi_line_string
        .0
        .iter()
        .map(|line_string| create_line_string_type(line_string))
        .collect()
}

pub fn create_polygon_type<T>(polygon: &Polygon<T>) -> PolygonType
where
    T: Float + Debug,
{
    let mut coords = vec![polygon
        .exterior()
        .points()
        .map(|point| create_point_type(&point))
        .collect()];

    coords.extend(
        polygon
            .interiors()
            .iter()
            .map(|line_string| create_line_string_type(&line_string)),
    );

    coords
}

pub fn create_multi_polygon_type<T>(multi_polygon: &MultiPolygon<T>) -> Vec<PolygonType>
where
    T: Float + Debug,
{
    multi_polygon
        .0
        .iter()
        .map(|polygon| create_polygon_type(&polygon))
        .collect()
}

#[allow(clippy::ptr_arg)]
pub fn create_geo_coordinate<T>(point_type: &PointType) -> Coord<T>
where
    T: Float + Debug,
{
    Coord {
        x: T::from(point_type[0]).unwrap(),
        y: T::from(point_type[1]).unwrap(),
    }
}

#[allow(clippy::ptr_arg)]
pub fn create_geo_point<T>(point_type: &PointType) -> Point<T>
where
    T: Float + Debug,
{
    Point::new(
        T::from(point_type[0]).unwrap(),
        T::from(point_type[1]).unwrap(),
    )
}

pub fn create_geo_multi_point<T>(multipoint_type: &[PointType]) -> MultiPoint<T>
where
    T: Float + Debug,
{
    multipoint_type.iter().map(create_geo_point).collect()
}

#[allow(clippy::ptr_arg)]
pub fn create_geo_line_string<T>(line_type: &LineStringType) -> LineString<T>
where
    T: Float + Debug,
{
    LineString(
        line_type
            .iter()
            .map(|point_type| create_geo_coordinate(point_type))
            .collect(),
    )
}

pub fn create_geo_multi_line_string<T>(multi_line_type: &[LineStringType]) -> MultiLineString<T>
where
    T: Float + Debug,
{
    MultiLineString(
        multi_line_type
            .iter()
            .map(|point_type| create_geo_line_string(&point_type))
            .collect(),
    )
}

#[allow(clippy::ptr_arg)]
pub fn create_geo_polygon<T>(polygon_type: &PolygonType) -> Polygon<T>
where
    T: Float + Debug,
{
    let exterior = polygon_type
        .get(0)
        .map(|e| create_geo_line_string(e))
        .unwrap_or_else(|| create_geo_line_string(&vec![]));

    let interiors = if polygon_type.len() < 2 {
        vec![]
    } else {
        polygon_type[1..]
            .iter()
            .map(|line_string_type| create_geo_line_string(line_string_type))
            .collect()
    };

    Polygon::new(exterior, interiors)
}

pub fn create_geo_multi_polygon<T>(multi_polygon_type: &[PolygonType]) -> MultiPolygon<T>
where
    T: Float + Debug,
{
    MultiPolygon(
        multi_polygon_type
            .iter()
            .map(|polygon_type| create_geo_polygon(&polygon_type))
            .collect(),
    )
}

pub fn create_geo_geometry_collection<T>(geometries: &[Geometry]) -> geo::GeometryCollection<T>
where
    T: Float + Debug,
{
    geo::GeometryCollection::from_iter(geometries.iter().map(|g| match &g.value {
        Value::Point(p) => geo::Geometry::Point(create_geo_point(&p)),
        Value::LineString(l) => geo::Geometry::LineString(create_geo_line_string(&l)),
        Value::Polygon(p) => geo::Geometry::Polygon(create_geo_polygon(&p)),
        Value::MultiPoint(p) => geo::Geometry::MultiPoint(create_geo_multi_point(&p)),
        Value::MultiPolygon(p) => geo::Geometry::MultiPolygon(create_geo_multi_polygon(&p)),
        Value::MultiLineString(p) => {
            geo::Geometry::MultiLineString(create_geo_multi_line_string(&p))
        }
        Value::GeometryCollection(g) => {
            geo::Geometry::GeometryCollection(create_geo_geometry_collection(&g))
        }
    }))
}
