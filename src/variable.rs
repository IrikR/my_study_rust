// 4. Переменные
//
// Значения (как и литералы) могут быть связаны с переменными, используя обозначение let.
pub mod variable {
    pub fn variable() {
        let an_integer = 1;
        let a_boolean = true;
        let unit = (1, 2, 3);

        // скопировать значение `an_integer` в `copied_integer`
        let copied_integer = an_integer;

        println!("An integer: {}", copied_integer);
        println!("A boolean: {}", a_boolean);
        println!("Meet the unit value: {}", unit.0);

        // Компилятор предупреждает о неиспользуемых переменных; эти предупреждения можно
        // отключить используя подчёркивание перед именем переменной
        let _unused_variable = 3;
        let _noisy_unused_variable = 2;
        // ИСПРАВЬТЕ ^ Добавьте подчёркивание
    }

    // 4.1 Изменяемость
    //
    // По умолчанию переменные нельзя изменять, но это можно исправить, добавив модификатор mut.

    pub fn variability() {
        let immutable_variable = 1;
        let mut mutable_variable = 1;
        println!("immutable_variable = {}", immutable_variable);

        println!("Before mutation: {}", mutable_variable);
        // Ок
        mutable_variable += 1;
        println!("After mutation: {}", mutable_variable);

        // Ошибка!
        // _immutable_variable += 1;
    }

    // Компилятор будет выводить сообщения об ошибке изменчивости.

    // 4.2 Области и видимость
    //
    // Переменные имеют локальную область, и имеют видимость в блоке (блок представляет
    // собой набор операторов, заключённых в фигурные скобки {}).
    // Кроме того, допускается скрытие переменной.

    pub fn area_of_visibility() {
        // Эта переменная живет в области функции main
        let long_lived_variable = 1;

        // Это блок, он имеет меньший объем нежели основная функция
        {
            // Эта переменная существует только в этом блоке
            let short_lived_variable = 2;

            println!("inner short: {}", short_lived_variable);

            // Эта переменная не видна внешней функции
            let long_lived_variable = 5_f32;

            println!("inner long: {}", long_lived_variable);
        }
        // Конец блока

        // Ошибка! `short_lived_variable` не существует в этой области
        // println!("outer short: {}", short_lived_variable);
        // ИСПРАВЬТЕ ^ Закомментируйте строку

        println!("outer long: {}", long_lived_variable);
    }

    // 4.3) Предварительное объявление.
    //
    // Можно сперва объявлять переменные, а инициализировать их позже.
    // Но эта форма редко используется, так как это может привести к использованию
    // неинициализированных переменных.

    pub fn pre_announcement() {
        // Объявляем переменную
        let a_variable;

        {
            let x = 2;

            // Инициализируем переменную
            a_variable = x * x;
        }

        println!("a variable: {}", a_variable);

        let another_variable;

        // Ошибка! Использование неинициализированной переменной
        // println!("another variable: {}", another_variable);
        // ИСПРАВЬТЕ ^ Закомментируйте строку

        another_variable = 1;

        println!("another variable: {}", another_variable);
    }

    // Компилятор запрещает использование неинициализированных переменных, так как это привело бы
    // к непредсказуемым последствиям.
}
