# Dokumentacja projektu

**Skład grupy**:
- Wojciech Dąbek - wdabek@student.agh.edu.pl
- Ida Ciepiela - idaciepiela@student.agh.edu.pl

Z grupy laboratoryjnej nr 5.

**Tytuł projektu**: *Space Crabs*

**Temat projektu**: Platforma do tworzenia i rezerwowania kosmicznych ekspedycji / wycieczek turystycznych.

**Wykorzystywany SZBD**: [MongoDB](https://www.mongodb.com/)

**Technologia realizacji aplikacji**: Język [Rust](https://www.rust-lang.org/) - framework [Rocket](https://rocket.rs/).

# Funkcjonalności

1. Każdy niezalogowany:
	- wyświetlać dostępne ekspedycje
	- zalogować się
2. Każdy zalogowany:
	- wyświetlać swoje dane logowania
3. Participant:
	- wyświetlać ekspedycje na które jest zapisany
	- zapisać się na ekspedycję
	- zapłacić za ekspedycję
4. Organizer:
	- utworzyć ekspedycję
	- zaakceptować rezerwacje uczestnika
5. Admin:
	- generowanie raportów
	- wyświetlanie danych użytkowników


# Struktura bazy daych

Zdecydowaliśmy się na bazę, która będzie się składać z dwóch kolekcji - _users_ i _expeditions_. 

## Kolekcja _users_

Kolekcja ta zawiera informacje o użytkownikach naszego serwisu. Każdy z nich posiada następujące pola:
- [_id]
- [login] - nazwę użytkownika 
- [password] - hasło 
- [role] - rolę 
Dodatkowo w zależności od tego jaką rolę będzie miał użytkownik takie dodatkowe pola będą w danym dokumencie. Dostępne role oraz ich pola prezentują się następująco:
1.  participant
	- [firstname] - imie
	- [lastname] - nazwisko
	- [p_expeditions] - ekspedycje, na które zapisany jest użytkownik. Każda ekspedycja jest obiektem o następujących polach:
		- [exp_id]
		- [name] - nazwa eksedycji
		- [start_date] - data rozpoczęcia
		- [reserved] - status rezerwacji
		- [paid] - status opłacenia rezerwacji
2. organizer
	- [company_name] - nazwa firmy, z którą powiązany jest użytkownik
	- [contact] - numer kontaktowy
	- [o_expeditions] - ekspedycje, którą są organizowane przez danego użytkownika. Każda ekspedycja jest obiektem o następujących polach:
		- [exp_id]
		- [name] - nazwa eksedycji
		- [start_date] - data rozpoczęcia
3. admin


Dany użytkownik może więcej niż jedną rolę.

Przykładowe dokumenty z kolekcji _users_:


```json
{
  "_id": {
    "$oid": "661e363d2e1f19d1b89964b3"
  },
  "login": "ala",
  "password": "makota",
  "role": ["participant"],
  "firstname": "Ala",
  "lastname": "Niemakota",
  "p_expeditions": [
    {
      "exp_id": "exp_id",
	  "name": "exp_name",
	  "start_date":2400,
      "reserved": false,
      "paid": false
    },
    {
      "exp_id": "exp_id2",
	  "name": "exp_name",
	  "start_date":2401,
      "reserved": true,
      "paid": true
    }
  ]
}
```


```json
{
  "_id": {
    "$oid": "661e39492e1f19d1b89964b7"
  },
  "login": "admin",
  "password": "admin",
  "role": "admin"
}
```

```json
{
  "_id": {
    "$oid": "661e39152e1f19d1b89964b5"
  },
  "login": "ola",
  "password": "mapsa",
  "role": ["organizer"],
  "company_name": "Olamakonia",
  "contact": "999-999-999",
  "o_expeditions": [
    {
      "exp_id": "exp_id1",
	  "name": "exp_name1",
	  "start_date":2400
    },
    {
      "exp_id": "exp_id2",
	  "name": "exp_name",
	  "start_date":2401
    }
  ]
}
```

```json
{
  "_id": {
    "$oid": "661e39152e1f19d1b89964b6"
  },
  "login": "jasiu",
  "password": "haslo1234",
  "firstname": "Jan",
  "lastname": "Kowalski",
  "role": ["organizer", "participant"],
  "company_name": "Crunchy Cola",
  "contact": "123-456-789",
  "p_expeditions": [
    {
      "exp_id": "234567890",
	  "name": "ABC",
	  "start_date":2421,
      "reserved": true,
      "paid": false
    },
    {
      "exp_id": "34567890987",
	  "name": "MoonRiding",
	  "start_date":2433,
      "reserved": true,
      "paid": true
    }
  ],
  "o_expeditions": [
    {
      "exp_id": "234567890",
	  "name": "Mars-o-Polo",
	  "start_date":2400
    }
  ]
}
```


## Kolekcja _expeditions_

Kolekcja ta zawiera informacje na temat ekspedycji
Każdy z dokumentów zawiera następujące pola:
- [_id]
- [name] - nazwa ekspedycji
- [stops] - lista przystanków podczas ekspedycji. Każdy przystanek jest obiektem o następujących polach:
	- [name] - nazwa przystanku
	- [arrival_time] - czas przylotu
	- [departure_time] - czas odlotu
- [max_no_participants] - maksymalna liczba uczestninków
- [guide] - dane przewodnika 
	- [firstname]
	- [lastname]
	- [age]
	- [experience]
- [organizer] - dane organizatora
	- [user_id] 
	- [company_name]
- [start_time] - czas rozpoczęcia 
- [end_time] - czas zakończenia
- [home_station] - stacja macierzysta
- [participants] - lista uczestników. Każdy z uczestników jest obiektem o następujących polach:
	- [user_id]
	- [firstname]
	- [lastname]
	- [paid] - status opłacenia rezerwacji
- [price] - cena ekspedycji


Przykładowe dokumenty z kolekcji _expeditions_:

```json
{
  "_id": {
    "$oid": "661e3a7d2e1f19d1b89964bb"
  },
  "name": "AAA",
  "stops": [
    {
      "name": "Pluto",
      "arrival_time": 2450,
      "departure_time": 2451,
    },
    {
      "name": "Proxima Centauri",
      "arrival_time": 2452,
      "departure_time": 2453,
    },
    {
      "name": "Earth",
      "arrival_time": 2455,
      "departure_time": 2456,
    }
  ],
  "max_no_participants": 200,
  "guide": {
    "firstname": "Tam",
    "lastname": "Rusty",
    "age": 8,
    "experience": "Capitan"
  },
  "organizer": {
	"user_id":"1234567890",
	"company_name":"Crunchy Cola"
  }, 
  "start_time": 2449,
  "end_time": 2460,
  "home_station": "Mars",
  "participants": [
	{
		"user_id":"345678",
		"firstname":"Ala",
		"lastname": "makota",
		"paid":false
	}
		{
		"user_id":"345678",
		"firstname":"Lorem",
		"lastname": "Ipsum",
		"paid":true
	}
  ],
  "price": 132222
}
```

