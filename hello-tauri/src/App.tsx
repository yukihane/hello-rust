import ja from "date-fns/locale/ja";
import React, { ChangeEvent, useState } from "react";
import DatePicker, { registerLocale } from "react-datepicker"; // https://github.com/Hacker0x01/react-datepicker
import "react-datepicker/dist/react-datepicker.css";

registerLocale("ja", ja);

function App() {
  const [name, setName] = useState("二宮尊徳");
  const [birthDay, setBirthDay] = useState(new Date("1787-09-04"));

  const handleNameChanged = (e: ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const handleBirthDayChanged = (date: Date) => {
    setBirthDay(date);
  };

  return (
    <div>
      <div style={{ padding: 10 }}>
        <div>
          <label htmlFor="name">名前</label>
          <div>
            <input
              type="text"
              name="name"
              id="name"
              value={name}
              onChange={handleNameChanged}
            />
          </div>
        </div>
        <div>
          <label htmlFor="birthday">生年月日</label>
          <DatePicker
            dateFormat="yyyy/MM/dd"
            locale="ja"
            selected={birthDay}
            onChange={handleBirthDayChanged}
          />
        </div>
        <input type="button" value="送信" />
      </div>
      <div style={{ padding: 10 }}>
        <input type="text" name="response" id="response" />
      </div>
    </div>
  );
}

export default App;
