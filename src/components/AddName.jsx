import React from "react";
import PropTypes from 'prop-types';
import Select from 'react-select';


export default function AddName({ onSubmit, setMonth, setDay}) {

  const DAY = [{value: 1, label: '1st'},
{value: 2, label: '2nd'},
{value: 3, label: '3rd'},
{value: 4, label: '4th'},
{value: 5, label: '5th'},
{value: 6, label: '6th'},
{value: 7, label: '7th'},
{value: 8, label: '8th'},
{value: 9, label: '9th'},
{value: 10, label: '10th'},
{value: 11, label: '11th'},
{value: 12, label: '12th'},
{value: 13, label: '13th'},
{value: 14, label: '14th'},
{value: 15, label: '15th'},
{value: 16, label: '16th'},
{value: 17, label: '17th'},
{value: 18, label: '18th'},
{value: 19, label: '19th'},
{value: 20, label: '20th'},
{value: 21, label: '21st'},
{value: 22, label: '22nd'},
{value: 23, label: '23rd'},
{value: 24, label: '24th'},
{value: 25, label: '25th'},
{value: 26, label: '26th'},
{value: 27, label: '27th'},
{value: 28, label: '28th'},
{value: 29, label: '29th'},
{value: 30, label: '30th'},
{value: 31, label: '31st'},
  ];

  const MONTH = [{value: 0, label: 'Jan'},
        {value: 1, label: 'Feb'},
        {value: 2, label: 'Mar'},
        {value: 3, label: 'Apr'},
        {value: 4, label: 'May'},
        {value: 5, label: 'Jun'},
        {value: 6, label: 'Jul'},
        {value: 7, label: 'Aug'},
        {value: 8, label: 'Sept'},
        {value: 9, label: 'Oct'},
        {value: 10, label: 'Nov'},
        {value: 11, label: 'Dec'},
  ];

  const onMonth = (option) => {
          setMonth(option);
  }

  const onDay = (option) => {
          setDay(option);
  }

  return (
    <>
    <form onSubmit={onSubmit}>
      <fieldset>
    <h2> Add a new birthday: </h2>

    <p className="highlight">
      <label htmlFor="newname">Name:</label>
      <input
        autoComplete="off"
        id="newname"
      />
    </p>

    <Select
        placeholder="Select Month"
        options={MONTH} // set list of the data
        onChange={onMonth} // assign onChange function
      />
    <Select
        placeholder="Select Option"
        options={DAY} // set list of the data
        onChange={onDay} // assign onChange function
      />

     <button type="Add"> Add </button>
       </fieldset>
     </form>
    </>
  );
}

AddName.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  setMonth: PropTypes.func.isRequired,
  setDay: PropTypes.func.isRequired,
};
