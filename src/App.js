import "regenerator-runtime/runtime";
import React, { useState, useEffect } from "react";
import PropTypes from "prop-types";
import Big from "big.js";
import LookupBirthday from "./components/LookupBirthday";
import Results from "./components/Results";
import * as nearAPI from 'near-api-js';

const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();
const { utils } = nearAPI;

const App = ({ contract, currentUser, nearConfig, wallet }) => {
  // use React Hooks to store names in component state
  const [names, setNames] = useState([]);

  const [selectedOption, setSelectedOption] = useState();
  const [upcoming, setUpcoming] = useState([]);

  useEffect(async () => {
      loadUpcoming();
    } else {
      loadResults();
    }
  },[allowVote]);

  const loadUpcoming = () => {
    contract.get_candidates()
      .then((candidates) => {
        let avail = [];
        candidates.map((candidate, i) =>
            avail.push({value: candidate, label: candidate})
        );
        setNames(avail);
      });
  };

  const loadResults = () => {
    contract.get_results().then((results) => {
      setResults(results);
    });
  };

  const beyondTheGrave = () => {
    console.log("you get another vote")
    contract.from_the_grave({
    },
      BOATLOAD_OF_GAS,
    ).then(() => {
      setAllowVote(true);
    });
  };

  const findByName = async event => {
    event.preventDefault();

    const { fieldset, writein } = event.target.elements;
    if (writein.value != "") {
      if (writein.value == "From the grave") {
        beyondTheGrave();
        return;
      }

      await contract.add_candidate({
              name: writein.value
      },
        BOATLOAD_OF_GAS,
      );
      setAllowVote(false);
      return;
    }


    if (selectedOption == null) {
      console.log("nothing selected");
      return;
    }
      contract.vote_for({
        name: selectedOption.value
      },
       BOATLOAD_OF_GAS,
      ).then(() => {
        setAllowVote(false);
        setSelectedOption(null);
      });
  };


  const userVoted = async event => {
    const res = await contract.user_voted({
      account_id: currentUser.accountId
    });
      if (res) {
        setAllowVote(false);
        loadResults();
        return;
      }
      setAllowVote(true);
      loadCandidates();
  };

  const signIn = () => {
    wallet.requestSignIn(
      nearConfig.contractName,
      "birthdays"
    ).then(() => {

    userVoted();
    });
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
      { currentUser
        <LookupBirthday onSubmit={findByName} names={names} setSelected={setSelectedOption}/>
      }
      </>
    </main>
  );
};

App.propTypes = {
  contract: PropTypes.shape({
    add: PropTypes.func.isRequired,
    remove: PropTypes.func.isRequired,
    get_all_birthdays_by_name: PropTypes.func.isRequired,
    get_all_birthdays_by_date: PropTypes.func.isRequired,
    get_birthdays_for_date: PropTypes.func.isRequired,
    get_birthdays_for_name: PropTypes.func.isRequired
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
