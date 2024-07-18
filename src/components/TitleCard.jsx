import Subtitle from "./typo/Subtitle"

  
  function TitleCard({title, children, topMargin, TopSideButtons,callRefresh}){
      return(
          <div className={"card w-full p-6 bg-base-100 shadow-xl " + (topMargin || "mt-6")}>

            {/* Title for Card */}
              <Subtitle styleClass={TopSideButtons ? "inline-block" : ""}>
                {title}

                {/* Top side button, show only if present */}
                {
                    TopSideButtons && <button className="btn btn-primary float-right" onClick={() => callRefresh()}>{TopSideButtons}</button>
                    // <button className="btn btn-primary float-right" onClick={() => updateProfile()}>Update</button>
                }
              </Subtitle>
              
              <div className="divider mt-2"></div>
          
              {/** Card Body */}
              <div className='h-full w-full pb-6 bg-base-100'>
                  {children}
              </div>
          </div>
          
      )
  }
  
  
  export default TitleCard