import { invoke } from "@tauri-apps/api";
import ja from "date-fns/locale/ja";
import React, { ChangeEvent, useState } from "react";
import DatePicker, { registerLocale } from "react-datepicker"; // https://github.com/Hacker0x01/react-datepicker
import "react-datepicker/dist/react-datepicker.css";

registerLocale("ja", ja);

type Request = {
  personalData: {
    name: string;
    birthDay: Date;
  };
};

type Response = {
  message: string;
};

function App() {
  const [name, setName] = useState("二宮尊徳");
  const [birthDay, setBirthDay] = useState(new Date("1787-09-04"));
  const [message, setMessage] = useState("");

  const handleNameChanged = (e: ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const handleBirthDayChanged = (date: Date) => {
    setBirthDay(date);
  };

  const submit = () => {
    const data: Request = { personalData: { name, birthDay } };
    console.log("send: " + JSON.stringify(data));
    invoke<Response>("greet", data).then((resp) => {
      console.log("recv:" + JSON.stringify(resp));
      setMessage(resp.message);
    });
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
        <input type="button" value="送信" onClick={submit} />
      </div>
      <div style={{ padding: 10 }}>
        <input
          readOnly
          type="text"
          name="response"
          id="response"
          value={message}
          style={{ width: 300 }}
        />
      </div>
    </div>
  );
}

export default App;
