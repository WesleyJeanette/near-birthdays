import React from 'react';
import PropTypes from 'prop-types';

export default function Results({ results }) {
  return (
    <>
      <h2>Voting Results:</h2>
      {results
         .sort((a, b) => a[1] < b[1] ? 1 : -1)
         .map((result, i) =>
        <p key={i} className="results">
          <strong>{result[0]}</strong>:  {result[1]}
        </p>
      )}
    </>
  );
}

Results.propTypes = {
  results: PropTypes.array.isRequired
};
