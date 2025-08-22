Cinnabar - Мой минималистичный язык прогррамирования

Cinnabar — это язык общего назначения с акцентом на лаконичность, строгую типизацию, метапрограммирование и поддержку как функционального, так и ООП-подходов.

Особенности:

Сильная и гибкая типизация (int, float, tuple, list, hash и др.)

Компиляторные вычисления (comptime)

Декораторы и макросы

Интерфейсы и реализации (OOP)

Лаконичный синтаксис (сахар через =>, take, shadowing правилa)

📦 Структура проекта

```
package "main" // имя пакета

plug std.io
plug std.panic

// Пример импорта с алиасом
plug libs.lib1:func1 as function
```

🔑 Типы данных

int (int8, int16, int32, int64, int128)

uint (uint8, uint16, uint32, uint64, uint128)

float (float32, float64)

bool, string, char

list — динамический массив (разные типы)

tuple — статический кортеж (разные типы)

array — статический массив (один тип)

hash — словарь (map)

hashlist — множество (set)

📝 Переменные

```
let x := 5;         // изменяемая переменная
set y := 3;         // неизменяемая
const z := 10;      // константа

let may_be_null?;   // nullable
```

Shadowing

```
set num1 := 6;
set num1 := map(float, num1); // shadowing
```

📌 Правила:

set → shadowing разрешено

let → меняется значение, но не тип

const → PANIC при переобъявлении

⚙️ Функции
Лямбда

```
set pow :int = lambda(a, b) => a ** b;
```

Обычная функция

```
void summator(a:int, b:int) {
    take a+b;
}
```

take и return взаимозаменяемы.

Generics

```
void multimmator<T: int | float>(a, b): Optional<int | float> { 
    a * b
}
```

🔀 Условные конструкции

```
match age {
    >= 18 => io.print("welcome");
    < 18  => io.print("go away");
    _     => throw panic.ValueError;
}
```

Короткая форма:

```
match name => == "Alice" => io.print("you Alice!");
```

❌ Ошибки и исключения

```
try {
    set res := num1 / num2;
    catch e => io.print($"error {e}");
    catch e == panic.ZeroDivision {
        io.print("НА НОЛЬ ДЕЛИТЬ НЕЛЬЗЯ");
    }
    finally => io.print("res = {res}");
}
```

🔁 Циклы
Бесконечный

```
loop {
    ...
}
```

С условием

```
loop { 
    let i := 0
    match i {
        >= 10 => break;
        _ => continue;
    }
    ...
}
```

📌 Управление: break, continue.

🐾 Объектная модель
Интерфейс

```
ifce Animals {
    void new(),
    void animal_voice() {
        io.print("some sound");
    },
    void fly_status(),
} 
```

Структура + реализация

```
scatch Animal {
    can_fly: bool,
    voice: string,

    impl Animal(Animals) {
        void $new(can_fly, voice):self { // здесь волшебный метод $new()
            self.can_fly = can_fly;
            self.voice = voice;
        }
    }
}
```

🧩 Метапрограммирование
Декораторы

```
@sppedtest(logging=true)
void long_func(wait: int) {
    io.print("func sleeps");
    time.sleep(wait);
    io.print("func awake");
} 
```

Макросы

```
make_macro! {
    macro add<T: int | float>(a, b) {
        quote {
            a + b
        }
    }
}
set x := add!(5, 3); // → 8
```

Compile-time вычисления

```
const SIZE := $comptime!(2 ** 5) // SIZE = 32;
```

⚡ Особенности синтаксиса

=> — короткая запись (лямбда, однострочные match/if)

take и return — равнозначные ключевые слова

Nil — null-значение

Метапрограммирование через декораторы, макросы, comptime


