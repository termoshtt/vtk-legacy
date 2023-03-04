use crate::*;
use nom::{bytes::complete::*, character::complete::*, combinator::opt, sequence::tuple, Parser};
fn name(input: &str) -> Result<&str> {
    take_till(|c: char| c.is_whitespace()).parse(input)
}

/// Scalars
///
/// Scalar definition includes specification of a lookup table.
/// The definition of a lookup table is optional.
/// If not specified, the default VTK table will be used (and tableName should be "default").
/// Also note that the numComp variable is optional by default the number of components is equal to one.
/// (The parameter numComp must range between (1,4) inclusive;
/// in versions of VTK prior to vtk2.3 this parameter was not supported.)
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Scalars {
    name: String,
    table_name: String,
    num_comp: u8,
    scalars: Data1D,
}

pub fn scalars(n: usize) -> impl FnMut(&str) -> Result<Scalars> {
    move |input| {
        let (input, (_tag, _, data_name, _, data_type)) =
            tuple((tag("SCALARS"), multispace1, name, multispace1, data_type)).parse(input)?;

        // num_comp or first element of scalars
        let (input, num_comp) =
            opt(tuple((multispace1, data1(data_type))).map(|x| x.1)).parse(input)?;

        let (input, table_name) = opt(tuple((multispace1, tag("LOOKUP_TABLE"), multispace1, name))
            .map(|(_, _tag, _, name)| name))
        .parse(input)?;

        let (input, (num_comp, scalars)) = if table_name.is_some() {
            // num_comp and scalars are explicitly separated in this case
            let (input, (_, scalars)) = tuple((multispace1, data1d(data_type, n))).parse(input)?;
            (input, (num_comp.map(|v| v.to_u8()).unwrap_or(1), scalars))
        } else {
            let num_comp = num_comp.unwrap();
            let (input, (_, mut scalars)) =
                tuple((multispace1, data1d(data_type, n - 1))).parse(input)?;
            // if the first scalar is parsed as `numComp`, this is `None`,
            // otherwise the last scalar.
            let (input, last) =
                opt(tuple((multispace1, data1(data_type))).map(|(_, last)| last)).parse(input)?;
            let num_comp = if let Some(last) = last {
                scalars.push(last);
                num_comp.to_u8()
            } else {
                scalars.insert(0, num_comp);
                1
            };
            (input, (num_comp, scalars))
        };

        Ok((
            input,
            Scalars {
                name: data_name.to_string(),
                table_name: table_name.unwrap_or("default").to_string(),
                num_comp,
                scalars,
            },
        ))
    }
}

/// ```text
/// COLOR_SCALARS dataName nValues
/// c00 c01 ... c 0(nValues-1)
/// c10 c11 ... c 1(nValues-1)
/// ...
/// c(n-1)0 c (n-1)1 ... c (n-1)(nValues-1)
/// ```
pub struct ColorScalars {}
pub fn color_scalars(_input: &str) -> Result<ColorScalars> {
    todo!()
}

/// ```text
/// LOOKUP_TABLE tableName size
/// r0 g0 b0 a0
/// r1 g1 b1 a1
/// ...
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LookupTable {
    table_name: String,
    colors: Vec<[f32; 4]>,
}

pub fn lookup_table(input: &str) -> Result<LookupTable> {
    let (input, (_tag, _, table_name, _, size, _)) = tuple((
        tag("LOOKUP_TABLE"),
        multispace1,
        name,
        multispace1,
        uint::<usize>,
        multispace1,
    ))
    .parse(input)?;
    let (input, colors) = take_4n::<f32>(size).parse(input)?;
    Ok((
        input,
        LookupTable {
            table_name: table_name.to_string(),
            colors,
        },
    ))
}

/// ```text
/// VECTORS dataName dataType
/// v0x v0y v0z
/// v1x v1y v1z
/// ...
/// v(n-1)x v(n-1)y v(n-1)z
/// ```
pub struct Vectors {}
pub fn vectors(_input: &str) -> Result<Vectors> {
    todo!()
}

/// ```text
/// NORMALS dataName dataType
/// n0x n0y n0z
/// n1x n1y n1z
/// ...
/// n(n-1)x n(n-1)y n(n-1)z
/// ```
pub struct Normals {}
pub fn normals(_input: &str) -> Result<Normals> {
    todo!()
}

/// ```text
/// TEXTURE_COORDINATES dataName dim dataType
/// t00 t01 ... t0(dim-1)
/// t10 t11 ... t1(dim-1)
/// ...
/// t(n-1)0 t(n-1)1 ... t(n-1)(dim-1)
/// ```
pub struct TextureCoordinates {}
pub fn texture_coordinates(_input: &str) -> Result<TextureCoordinates> {
    todo!()
}

/// ```text
/// TENSORS dataName dataType
/// t000 t001 t002
/// t010 t011 t012
/// t020 t021 t022
///
/// t100 t101 t102
/// t110 t111 t112
/// t120 t121 t122
///
/// ...
///
/// t(n-1)00 t(n-1)01 t(n-1)02
/// t(n-1)10 t(n-1)11 t(n-1)12
/// t(n-1)20 t(n-1)21 t(n-1)22
/// ```
pub struct Tensors {}
pub fn tensors(_input: &str) -> Result<Tensors> {
    todo!()
}

/// ```text
/// FIELD dataName numArrays
/// arrayName0 numComponents numTuples dataType
/// f00 f 01 ... f 0(numComponents-1)
/// f10 f 11 ... f 1(numComponents-1)
/// ...
/// f(numTuples-1)0 f(numTuples-1)1 ... f(numTuples-1)(numComponents-1)
///
/// arrayName1 numComponents numTuples dataType
/// f00 f 01 ... f 0(numComponents-1)
/// f10 f 11 ... f 1(numComponents-1)
/// ...
/// f(numTuples-1)0 f(numTuples-1)1 ... f(numTuples-1)(numComponents-1)
/// ...
///
/// arrayName(numArrays-1) numComponents numTuples dataType
/// f00 f 01 ... f 0(numComponents-1)
/// f10 f 11 ... f 1(numComponents-1)
/// ...
/// f(numTuples-1)0 f(numTuples-1)1 ... f(numTuples-1)(numComponents-1)
/// ```
pub struct Field {}
pub fn field(_input: &str) -> Result<Field> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::{LookupTable, Scalars};
    use crate::Data1D;
    use nom::{Finish, Parser};

    #[test]
    fn scalars() {
        let (residual, out) = super::scalars(6)
            .parse(
                r#"
                SCALARS cell_scalars int 1
                LOOKUP_TABLE default
                0 1 2 3 4 5
                "#
                .trim(),
            )
            .finish()
            .unwrap();
        assert_eq!(residual, "");
        assert_eq!(
            out,
            Scalars {
                name: "cell_scalars".to_string(),
                table_name: "default".to_string(),
                num_comp: 1,
                scalars: Data1D::Int(vec![0, 1, 2, 3, 4, 5])
            }
        );

        // omit LOOKUP_TABLE
        let (residual, out) = super::scalars(6)
            .parse(
                r#"
                SCALARS cell_scalars int 1
                0 1 2 3 4 5
                "#
                .trim(),
            )
            .finish()
            .unwrap();
        assert_eq!(residual, "");
        assert_eq!(
            out,
            Scalars {
                name: "cell_scalars".to_string(),
                table_name: "default".to_string(),
                num_comp: 1,
                scalars: Data1D::Int(vec![0, 1, 2, 3, 4, 5])
            }
        );

        // omit both LOOKUP_TABLE and numComp
        //
        // In this case, `0` in below input must be regared as
        // the first element of scalars, not as `numComp`.
        let (residual, out) = super::scalars(6)
            .parse(
                r#"
                SCALARS cell_scalars int
                0 1 2 3 4 5
                "#
                .trim(),
            )
            .finish()
            .unwrap();
        assert_eq!(residual, "");
        assert_eq!(
            out,
            Scalars {
                name: "cell_scalars".to_string(),
                table_name: "default".to_string(),
                num_comp: 1,
                scalars: Data1D::Int(vec![0, 1, 2, 3, 4, 5])
            }
        );
    }

    #[test]
    fn lookup_table() {
        let (residual, out) = super::lookup_table(
            r#"
            LOOKUP_TABLE my_table 8
            0.0 0.0 0.0 1.0
            1.0 0.0 0.0 1.0
            0.0 1.0 0.0 1.0
            1.0 1.0 0.0 1.0
            0.0 0.0 1.0 1.0
            1.0 0.0 1.0 1.0
            0.0 1.0 1.0 1.0
            1.0 1.0 1.0 1.0
            "#
            .trim(),
        )
        .finish()
        .unwrap();
        assert_eq!(residual, "");
        assert_eq!(
            out,
            LookupTable {
                table_name: "my_table".to_string(),
                colors: vec![
                    [0.0, 0.0, 0.0, 1.0],
                    [1.0, 0.0, 0.0, 1.0],
                    [0.0, 1.0, 0.0, 1.0],
                    [1.0, 1.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0, 1.0],
                    [1.0, 0.0, 1.0, 1.0],
                    [0.0, 1.0, 1.0, 1.0],
                    [1.0, 1.0, 1.0, 1.0],
                ]
            }
        );
    }
}
