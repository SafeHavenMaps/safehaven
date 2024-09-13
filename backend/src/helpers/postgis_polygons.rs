use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct MultiPolygon(Vec<Vec<(f64, f64)>>);

impl MultiPolygon {
    pub fn to_polygon_string(&self, srid: Option<u32>) -> String {
        let polygons_str = self
            .0
            .iter()
            .map(|polygon| {
                let coords_str = polygon
                    .iter()
                    .map(|&(x, y)| format!("{} {}", x, y))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", coords_str)
            })
            .collect::<Vec<_>>()
            .join("),(");

        match srid {
            Some(srid) => format!("SRID={};MULTIPOLYGON(({}))", srid, polygons_str),
            None => format!("MULTIPOLYGON(({}))", polygons_str),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_polygon_string() {
        let multi_polygon = MultiPolygon(vec![
            vec![(30.0, 20.0), (45.0, 40.0), (10.0, 40.0), (30.0, 20.0)],
            vec![
                (15.0, 5.0),
                (40.0, 10.0),
                (10.0, 20.0),
                (5.0, 10.0),
                (15.0, 5.0),
            ],
        ]);

        let expected_string =
            "MULTIPOLYGON(((30 20, 45 40, 10 40, 30 20)),((15 5, 40 10, 10 20, 5 10, 15 5)))";
        assert_eq!(multi_polygon.to_polygon_string(None), expected_string);
    }

    #[test]
    fn test_to_polygon_string_with_srid() {
        let multi_polygon = MultiPolygon(vec![
            vec![(30.0, 20.0), (45.0, 40.0), (10.0, 40.0), (30.0, 20.0)],
            vec![
                (15.0, 5.0),
                (40.0, 10.0),
                (10.0, 20.0),
                (5.0, 10.0),
                (15.0, 5.0),
            ],
        ]);

        let expected_string =
            "SRID=3857;MULTIPOLYGON(((30 20, 45 40, 10 40, 30 20)),((15 5, 40 10, 10 20, 5 10, 15 5)))";
        assert_eq!(multi_polygon.to_polygon_string(Some(3857)), expected_string);
    }
}
