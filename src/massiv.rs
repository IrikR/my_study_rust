pub mod mod_massiv {
    pub fn fn_massiv() {
        println!("модуль массив");
        // определение массива
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        println!("вывод массива целиком: {:?}", array); // выводит весь массив
        println!("вывод 3-го элемента массива{}", array[3]); // выводит конкретный элемент массива

        // генерирует массив из 2 в количестве 50 штук
        let array_1 = [2; 50];
        println!("вывод массива целиком: {:?}", array_1);

        let array_2 = [11, 22, 33, 44, 55, 66, 77, 88, 99];
        println!("длинна массива array_2: {}", array_2.len());

        // вывод массива с помощью for
        println!("вывод массива с помощью for");
        for i in array_2.iter() {
            println!("элемент массива: {}", i);
        }

        // вывод массива с помощью for
        println!("вывод массива с помощью for");
        for k in 0..array_2.len() {
            println!("последовательность массива: {}", array_2[k]);
        }

        // вывод массива с помощью while
        println!("вывод массива с помощью while");
        let mut l = 0;
        while l < array_2.len() {
            println!("последовательность массива: {}", array_2[l]);
            l += 1;
        }

        // выборка из массива четных чисел.
        println!("выборка из массива четных чисел.");
        for m in array.iter() {
            if m % 2 == 0 {
                println!("четные числа массива: {}", m);
            }
        }

        let array_3 = [1, 2, 2, 3, 4, 4, 5, 6, 7, 7, 8, 9, 9, 10];
        let mut n = 0;
        while n < array_3.len() {
            let mut j = n + 1;
            while j < array_3.len() {
                if array_3[n] == array_3[j] {
                    println!("совпадение: {}", array_3[n]);
                }
                j += 1;
            }
            n += 1;
        }
    }
}
