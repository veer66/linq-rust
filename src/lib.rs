macro_rules! query {
    (select $select_expr:expr; from $data_source:ident;) => {
        $data_source.map($select_expr)
    };
    (select $select_expr:expr; from $data_source:ident; where $where_expr:expr;) => {
        $data_source.filter($where_expr).map($select_expr)
    }; 
}

#[cfg(test)]
mod tests {
    #[test]
    fn select_query() {
        let x = 1..100;
        let y: Vec<_> = x.clone().map(|i| i * 2).collect();
        let z: Vec<_> = query!(select |i| i * 2; from x;).collect();
        assert_eq!(z, y);
    }

    #[test]
    fn where_query() {
        let x = 1..100;
        let y: Vec<_> = x.clone().filter(|&i| i % 2 == 0).map(|i| i * 2).collect();
        let z: Vec<_> = query!(select |i| i * 2; from x; where |i| i % 2 == 0;).collect();
        assert_eq!(z, y);
    }
    

    #[test]
    fn multiple_conditions_in_where_query() {
        let x = 1..100;
        let y: Vec<_> = x.clone().filter(|&i| i % 2 == 0 && i % 4 == 0).map(|i| i * 2).collect();
        let z: Vec<_> = query!(select |i| i * 2; from x; where |i| i % 2 == 0 && i % 4 == 0;).collect();
        assert_eq!(z, y);
    }

}
