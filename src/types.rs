// 5) Типы
//
// Rust обеспечивает безопасность с помощью статической проверки типов.
// Тип переменной может быть явно указан при её объявлении.
// Тем не менее, в большинстве случаев, компилятор сможет определить тип переменной
// из контекста, если это очевидно.

pub mod types {
    pub fn types() {
        // Аннотированный тип переменной
        let _a_float: f64 = 1.0;

        // Эта переменная типа `int`
        let _an_integer = 5;

        // Ошибка! Тип переменной нельзя изменять
        // an_integer = true;
    }

    // Это краткое изложение примитивных типов в Rust:
    //
    //     целые числа: i8, i16, i32, i64 и int (размер зависит от платформы)
    //     целые числа без знака: u8, u16, u32, u64 и uint (размер зависит от платформы)
    //     с плавающей точкой: f32, f64
    //     char значения Unicode: 'a', 'α' и '∞' (4 байта каждый)
    //     bool true или false
    //     кортежи ()

    // 5.1 Приведение типов
    //
    // Rust не предоставляет неявного преобразования типов (coercion) между примитивами,
    // но, явное приведение типов (casting) может быть достигнуто с помощью ключевого слова as.

    pub fn cast() {
        let decimal = 65.4321_f32;

        // Ошибка! Нет неявного преобразования
        // let integer: u8 = decimal;
        // ИСПРАВЬТЕ ^ Закомментируйте строку

        // Явное преобразование
        let integer = decimal as u8;
        let character = integer as char;

        println!("Casting: {} -> {} -> {}", decimal, integer, character);
    }

    // 5.2 Литералы
    //
    // В числовых литералах тип может быть аннотирован, добавив тип в качестве суффикса,
    // за исключением uint, использующей суффикс u и int, который использует суффикс i.
    //
    // Тип литералов без суффикса будет зависеть от того, как они используются.
    // Если никаких ограничений не существует, то компилятор выдаст сообщение об ошибке.

    pub fn literals() {
        // Литералы с суффиксами, их вид известен при инициализации
        let x = 1u8;
        let y = 2u16;
        let z = 3f32;

        // Литералы без суффикса, их вид зависит от того, как они используются
        let i = 1;
        let f = 1.0;

        // `size_of_val` возвращает размер переменной в байтах
        println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

        // Ограничения (слагаемые должны иметь тот же тип) для `i` и `f`
        let _constraint_i = x + i;
        let _constraint_f = z + f;
        // Закомментируйте эти две строки
    }

    // Есть некоторые понятия, используемые в предыдущем коде, которые не были объяснены раньше,
    // вот краткое объяснение для нетерпеливых читателей:
    //
    //     fun(&foo) используется, чтобы передать аргумент в функцию по ссылке,
    //      а не по значению fun(foo).
    //     std::mem::size_of_val является функцией, но вызывается с указанием полного пути.
    //      Код можно разделить на логические единицы, называемые модулями. Здесь функция size_of_val
    //      определена в модуле mem, а модуль mem определен в крэйте std.

    // 5.3) Логический вывод
    //
    // Логический вывод типов довольно умён. Тип добавляемой переменной используется как
    // определитель типа для второй переменной. Вот продвинутый пример:

    pub fn inference() {
        // Использование локального вывода, компилятор знает, что `elem` имеет тип `u8`
        let elem = 5u8;

        // Создадим пустой вектор (расширяемый массив)
        let mut vec = Vec::new();
        // В этот момент компилятор не знает точный тип `vec`, он
        // просто знает, что это вектор `Vec<_>`

        // Вставим `elem` в вектор
        vec.push(elem);
        // Ага! Теперь компилятор знает, что `vec` это вектор `u8` (`Vec<u8>`)
        // Попробуйте закомментировать строку `vec.push(elem)`

        println!("{}", vec[0]);
    }

    // Отсутствует необходимость в аннотации типа переменной, компилятор счастлив как и программист!

    // 5.4 Псевдонимы (алиасы)
    //
    // Оператор type может быть использован, чтобы задать новое имя существующему типу.
    // Тип должен быть в стиле CamelCase, либо компилятор выдаст предупреждение.
    // Исключением из этого правила являются примитивные типы: uint, f32 и другие.

    // `NanoSecond` это новое имя для `u64`
    // type NanoSecond = u64;
    // type Inch = u64;

    // Используйте этот атрибут, чтобы не выводить предупреждение
    // #[allow(non_camel_case_types)]
    // type uint64_t = u64;
    // Попробуйте удалить атрибут

    // pub fn alias() {
    //     // `NanoSecond` = `Inch` = `uint64_t` = `u64`
    //     let nanoseconds: NanoSecond = 5 as uint64_t;
    //     let inches: Inch = 2 as uint64_t;
    //
    //     // Обратите внимание, что псевдонимы новых типов не предоставляют
    //     // дополнительную безопасность, из-за того, что они не нового типа
    //     println!("{} nanoseconds + {} inches = {} unit?",
    //              nanoseconds,
    //              inches,
    //              nanoseconds + inches);
    // }

    fn print_me(msg: &str) {
        println!("msg = {}", msg);
    }

    pub fn fn_string() {
        let string = "привет, мир";
        print_me(string);

        let owned_string = "привет, мир".to_string(); // или String::from_str("привет, мир")
        print_me(&owned_string);

        let counted_string = std::rc::Rc::new("привет, мир".to_string());
        print_me(&counted_string);

        let atomically_counted_string = std::sync::Arc::new("привет, мир".to_string());
        print_me(&atomically_counted_string);
    }
}
