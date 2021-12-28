import React from 'react';
import * as moment from 'moment';
import PropTypes from 'prop-types';

export default function UpcomingBirthdays({ birthdays }) {
  return (
    <>
      <h2>Upcoming Birthdays:</h2>
      {birthdays
         .sort((a, b) => a[2] < b[2] ? 1 : -1)
         .map((result, i) =>
        <p key={i} className="results">
          <strong>{result[0]}</strong>:  {result[1]}
        </p>
      )}
    </>
  );
}

UpcomingBirthdays.propTypes = {
  birthdays: PropTypes.array.isRequired
};
