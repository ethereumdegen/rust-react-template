 

function Main(props) {
  
  
  return (
    <div
      className= {` p-1 mx-2 text-xs border-2 border-slate-400 rounded select-none cursor-pointer inline-block ${ props.customClass }`}
      onClick={() => {props.clicked ? props.clicked() : undefined}}
    > 
      {props.children}
    </div>

  );
}
/*
Main.propTypes = {
  width: PropTypes.oneOfType([PropTypes.number, PropTypes.string]),
  height: PropTypes.oneOfType([PropTypes.number, PropTypes.string]),
  lineColor: PropTypes.string,
  className: PropTypes.string,
};

Main.defaultProps = {
  width: "auto",
  height: "auto",
  lineColor: "",
  className: "",
};*/

export default Main;
