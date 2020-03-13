# tachyon
A web-templating system with a RESTful API

По ![ссылке](http://35.228.134.188:5001) можно ознакомится с функционированием сервиса

# Лабораторная работа №1
### 1. Идентификатор прецедента

Система веб шаблонов

### 2. Название прецедента

Разработка системы веб-шаблонов с RESTFul API, и графическим веб-интерфейсом

### 3. Контекст
Разработка почтовых рассылок

### 4. Участники (actors) и цели (goals)

| Участник  | Категория  | Цель (goal) |
|---|---|---|
| Программист-разработчик | Основной  | Разработка системы веб-шаблонов |
| Дизайнер | Внешний  | Разработка шаблонов |
| Прикладной программист | Внешний  | Использование api предоставлемой системой веб шаблонов |
| Tokio  | Инструмент  | библиотека асинхронного ввода/вывода для увеличения системы веб шаблонов |
| Tera | Инструмент | библиотека предоставлющие основные инструменты для шаблонизации |
| Vue.js | Инструмент | фреймворк для создания веб-интерфейсов |
| Google compute cloud  | Инструмент| Предоставляет вычислительные мощности для размещения системы веб шаблонов |

### 5. Предусловия (pre-conditions)

* изучен язык Rust (основной язык написания системы веб-шаблонов)
* изучены основные принципы работы системы веб шаблонов
* изучены приёмы и методики построения веб-интерфейсов

### 6. Постусловия (post-conditions)

* Разработана система веб-шаблонов
* Сделан веб-интерфейс для управления системой
* Разработан RESTFul API для взаимодействия с системой веб шаблонов
* Дизайнер может разработывать шаблоны веб-документов
* Прикладной программист может


### 7. Основной поток (main flow)

| Участник  | Действие (activity)  | Ожидаемый результат |
|---|---|---|
| Программист | Разрабатывает систему веб шаблонов | Система веб шаблонов |
| Программист | Разрабатывает RESTFul API для системы веб шаблонов | RESTFul API для системы веб шаблонов  |
| Программист | Разрабатывает веб интерфейс для управления системой веб-шаблонов | Веб интерфейс для управления системой веб-шаблонов |
| Дизайнер | Разработывает необходимые веб шаблоны | Готовые веб-шаблоны |
| Прикладной программист | Использует API системы веб-шаблонов для генерации веб-документов | Веб-документ |

### 8. Исключения (exceptions)
> Что может пойти не так?

| Условие (риск) | Последствия | Реакция |
|---|---|---|
| Сбой хостинга | Недоступная система веб шаблонов | Развёртка системы на резервном хостинге |

### 9. Альтернативы (alternates)
Для выполнения работы используется фреймворк Vue, вместо Angular для создания веб-интерфейса
Вместо github, в качестве хостинга используется google compute cloud

### 10. Временные параметры

* Триггер (событие, стартующее прецедент): Необходимость защищать выпускную квалификационную работу

* Номинальная частота повторения прецедента: Раз в жизнь + по мере необходимости

* Продолжительность прецедента: 40 ак.часа = 30 нормочаса

### 11. Диаграмма uml use case
![none](https://github.com/bobs4462/tachyon/blob/master/docs/images/uml_use_case.png)
# Лабораторная работа №2

### 1. Диаграмма uml use class
![none](https://github.com/bobs4462/tachyon/blob/master/docs/images/uml_class.png)


