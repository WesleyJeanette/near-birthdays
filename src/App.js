import "regenerator-runtime/runtime";
import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import Big from "big.js";
import moment from 'moment';
import LookupBirthday from "./components/LookupBirthday";
import UpcomingBirthdays from "./components/UpcomingBirthdays";
import AddName from "./components/AddName";
import Results from "./components/Results";
import * as nearAPI from 'near-api-js';

const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();
const { utils } = nearAPI;
const LOOK_AHEAD = 7;

const App = ({ contract, currentUser, nearConfig, wallet }) => {
  // use React Hooks to store names in component state
  const [names, setNames] = useState([]);
  const [upcoming, setUpcoming] = useState([]);

  const [selectedName, setSelectedName] = useState();
  const [selectedMonth, setSelectedMonth] = useState();
  const [selectedDay, setSelectedDay] = useState();
  const [results, setResults] = useState([]);

  useEffect(async () => {
      loadNames();
  },[]);

  const findByName = async event => {
    event.preventDefault();

    if (selectedName == null) {
      console.log("nothing selected");
      return;
    }
    let bday = await contract.get_birthday_for_name({
      name: selectedName.value
    });
    setResults({name: selectedName.value, date: bday});
  };

  const removeByName = async event => {
    if (selectedName == null) {
      console.log("nothing selected");
      return;
    }
    contract.remove({
      name: results.name,
      date: results.date
    },
    BOATLOAD_OF_GAS,
    );
    setResults(null);
    resetAll();
  };

  const lookahead = (name, upcoming) => {

    let today = Number(moment().format("DDD"));
    let look_ahead = Number(moment().add(LOOK_AHEAD, 'days').format("DDD"));
    let myDate = moment(name[1], ["MMM Do"], true);
    let date_of_year = Number(myDate.format("DDD"));
    if (today <= date_of_year && (date_of_year <= look_ahead || today + LOOK_AHEAD >= 365)) {
      upcoming.push({name: name[0], birthday: name[1], day: date_of_year})
    }
    // end of year roll over
    if (look_ahead < LOOK_AHEAD && date_of_year <= look_ahead) {
      upcoming.push({name: name[0], birthday: name[1], day: date_of_year})
    }
  }

  const loadNames = async event => {
    let names = await contract.get_all_birthdays();
    let avail = [];

    names.map((name, i) => avail.push({value: name[0], label: name[0]}))
    setNames(avail);

    let upcoming = [];
    names.map((name, i) => lookahead(name, upcoming));
    setUpcoming(upcoming);
  };

  const resetAll = () => {
   setSelectedName(null);
   setSelectedMonth(null);
   setSelectedDay(null);
  }

  const addName = async event => {
    event.preventDefault();

    const { fieldset, newname } = event.target.elements;
    if (newname.value == "") {
      console.log("nothing selected");
      resetAll();
      return;
    }
    if (selectedMonth == null) {
      console.log("nothing selected");
      resetAll();
      return;
    }
    if (selectedDay == null) {
      console.log("nothing selected");
      resetAll();
      return;
    }
    let myDate = moment().month(selectedMonth.value).date(selectedDay.value);
    console.log(myDate);
    console.log(selectedMonth.value);
    console.log(selectedDay.value);
// new date as a moment
    let bday = await contract.add({
      name: newname.value,
      date: myDate.format("MMM Do")
    },
    BOATLOAD_OF_GAS,
    );
    resetAll();
  };


  const signIn = () => {
    wallet.requestSignIn(
      nearConfig.contractName,
      "vote"
    );
  };

  const signOut = () => {
    wallet.signOut();
    window.location.replace(window.location.origin + window.location.pathname);
  };

  return (
    <main>
      <header>
        {currentUser &&
          <p>Currently signed in as: <code>{currentUser.accountId}</code></p>
        }

        { currentUser
          ? <button onClick={signOut}>Log out</button>
          : <div>
              <p> Please log in to update and see your birthday list</p>
              <button onClick={signIn}>Log in</button>
            </div>
        }
      </header>
      <>
      { currentUser &&
        <LookupBirthday onSubmit={findByName} names={names} setSelected={setSelectedName}/>
      }
      </>
      <>
      { currentUser && results.name
        ? <div>
        <p> {results.date}</p>
        <button onClick={removeByName}>Remove {results.name}</button>
        </div>
        : <p> Select name to see birthday</p>
      }
      </>
      <>
      { currentUser &&
        <AddName onSubmit={addName} setMonth={setSelectedMonth} setDay={setSelectedDay}/>
      }
      </>
      <>
      { currentUser &&
        <UpcomingBirthdays birthdays={upcoming}/>
      }
      </>
    </main>
  );
};

App.propTypes = {
  contract: PropTypes.shape({
    add: PropTypes.func.isRequired,
    remove: PropTypes.func.isRequired,
    get_birthdays_for_date: PropTypes.func.isRequired,
    get_birthday_for_name: PropTypes.func.isRequired,
    get_all_birthdays: PropTypes.func.isRequired,
  }).isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  }),
  nearConfig: PropTypes.shape({
    contractName: PropTypes.string.isRequired
  }).isRequired,
  wallet: PropTypes.shape({
    requestSignIn: PropTypes.func.isRequired,
    signOut: PropTypes.func.isRequired
  }).isRequired
};

export default App;
