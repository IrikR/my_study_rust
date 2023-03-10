pub mod arith {
    pub fn arith() {
        // Рассмотрим бинарные арифметические операции:

        // + Операция сложения возвращает сумму двух чисел:
        let num_0 = 12 + 6; // 18
        println!("12 + 6 = {}", num_0);
        // - Операция вычитания возвращает разность двух чисел:
        let num_1 = 12 - 6; // 6
        println!("12 - 6 = {}", num_1);
        // * Операция умножения возвращает произведение двух чисел:
        let num_2 = 12 * 6; // 72
        println!("12 * 6 = {}", num_2);
        // / Операция деления возвращает частное двух чисел:

        let num_3 = 12 / 6; // 2
        println!("12 / 6 = {}", num_3);
        // Стоит отметить, что если оба операнда операции представляют целые числа,
        // то при делении от дробного результата возвращается только целая часть:

        let num_4 = 29 / 6; // number = 4
        println!("29 / 6 = {} (i32)", num_4);
        let num_4: f32 = 29.0 / 6.0; // number = 4
        println!("29.0 / 6.0 = {} (f32)", num_4);
        // % Операция получения остатка от целочисленного деления:
        let num_5 = 15 % 6; // 3
        println!("15 % 6 = {}", num_5);
        // Ряд операций сочетают арифметические операции и операцию присваивания (=):

        // += Присваивание после сложения. Присваивает левому операнду сумму левого и правого
        // операндов: A += B эквивалентно A = A + B.
        let mut num_6 = 12;
        num_6 += 6; // number = 18
        println!("num += 6, num = 12, = {}", num_6);
        // -= Присваивание после вычитания.
        // Присваивает левому операнду разность левого и правого операндов:
        // A -= B эквивалентно A = A - B.
        let mut num_7 = 12;
        num_7 -= 6; // number = 6
        println!("num -= 6, num = 12, = {}", num_7);
        // *= Присваивание после умножения.
        // Присваивает левому операнду произведение левого и правого операндов:
        // A *= B эквивалентно A = A * B
        let mut num_8 = 12;
        num_8 *= 6; // number = 72
        println!("num *= 6, num = 12, = {}", num_8);
        // /= Присваивание после деления.
        // Присваивает левому операнду частное левого и правого операндов:
        // A /= B эквивалентно A = A / B
        let mut num_9 = 12;
        num_9 /= 6; // number = 2
        println!("num /= 6, num = 12, = {}", num_9);
        // %= Присваивание после деления по модулю.
        // Присваивает левому операнду остаток от целочисленного деления левого операнда на правый:
        // A %= B эквивалентно A = A % B
        let mut num_10 = 15;
        num_10 %= 6; // number = 3
        println!("num %= 6, num = 15, = {}", num_10);
    }
}
