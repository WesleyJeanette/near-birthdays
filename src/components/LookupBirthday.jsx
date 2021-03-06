import React from "react";
import PropTypes from 'prop-types';
import Select from 'react-select';


export default function LookupBirthday({ onSubmit, names, setSelected}) {

  const onChange = (option) => {
          setSelected(option);
  }

  return (
    <>
    <form onSubmit={onSubmit}>
      <fieldset>
    <h2> Look up Birthday by name: </h2>


    <Select
        placeholder="Select Option"
        options={names} // set list of the data
        onChange={onChange} // assign onChange function
      />

     <button type="submit"> Search </button>
       </fieldset>
     </form>
    </>
  );
}

LookupBirthday.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  names: PropTypes.array.isRequired,
  setSelected: PropTypes.func.isRequired
};
