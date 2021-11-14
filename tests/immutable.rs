use select_indices::prelude::*;

#[test]
fn immutable_select_indices_test()
{
    let data = [
        0,0,2,0,1,
        0,0,0,0,0,
        0,0,4,5,3
    ];

    let indices = [4, 2, 14, 12, 13];

    assert!(
        data.select_indices(&indices).eq(&[1, 2, 3, 4, 5])
    )
}

#[test]
#[should_panic]
fn immutable_out_of_range_panic()
{
    let data = [1,2,3];
    let indices = [1,2,3]; 

    data.select_indices(&indices) // 3 is out of bounds: should panic
        .eq(&[1,2,3]);
}

#[test]
#[should_panic]
fn immutable_repeated_index_panic()
{
    let data = [1,2,3];
    let indices = [1,1];

    data.select_indices(&indices) // Repeated index: should panic
        .eq(&[2,2]);
}

#[cfg(feature = "rayon")]
mod rayon
{
    use select_indices::prelude::*;
    use rayon::prelude::*;

    #[test]
    fn par_immutable_select_indices_test()
    {
        let data = [
            0,0,2,0,1,
            0,0,0,0,0,
            0,0,4,5,3
        ];

        let indices = [4, 2, 14, 12, 13];

        assert!(
            data.par_select_indices(&indices).eq(&[1, 2, 3, 4, 5])
        );
    }

    #[test]
    #[should_panic]
    fn par_immutable_out_of_range_panic()
    {
        let data = [1,2,3];
        let indices = [1,2,3]; 

        data.par_select_indices(&indices) // 3 is out of bounds: should panic
            .eq(&[1,2,3]);
    }

    #[test]
    #[should_panic]
    fn par_immutable_repeated_index_panic()
    {
        let data = [1,2,3];
        let indices = [1,1];

        data.par_select_indices(&indices) // Repeated index: should panic
            .eq(&[2,2]);
    }
}