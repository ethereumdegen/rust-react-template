 
function Main() {
  return (
    <>
      <div className="intro-y flex flex-col sm:flex-row items-center mt-8">
        <h2 className="text-lg font-medium mr-auto">Timeline</h2>
      </div>
      <div className="box p-5 intro-y mt-5">
        {/* BEGIN: Timeline Wrapper */}
        <div className="pb-20 px-5 mt-5 -mb-5 pb-5 relative overflow-hidden before:content-[''] before:absolute before:w-px before:bg-slate-200/60 before:dark:bg-darkmode-400 before:mr-auto before:left-0 lg:before:right-0 before:ml-3 lg:before:ml-auto before:h-full before:mt-8">
          <div className="relative z-10 bg-white dark:bg-darkmode-600 py-2 my-5 text-center text-slate-500 text-xs">
            March, 2022
          </div>
          {/* BEGIN: Timeline Content Latest */}
          <div className="lg:ml-[51%] pl-6 lg:pl-[51px] before:content-[''] before:absolute before:w-20 before:h-px before:mt-8 before:left-[60px] before:bg-slate-200 before:dark:bg-darkmode-400 before:rounded-full before:inset-x-0 before:mx-auto before:z-[-1]">
            <div className="bg-white dark:bg-darkmode-400 shadow-sm border border-slate-200 rounded-md p-5 flex flex-col sm:flex-row items-start gap-y-3 mt-10 before:content-[''] before:absolute before:w-6 before:h-6 before:bg-primary/20 before:rounded-full before:inset-x-0 lg:before:ml-auto before:mr-auto lg:before:animate-ping after:content-[''] after:absolute after:w-6 after:h-6 after:bg-primary after:rounded-full after:inset-x-0 lg:after:ml-auto after:mr-auto after:border-4 after:border-white/60 after:dark:border-darkmode-300">
              <div className="mr-3">
                <div className="image-fit w-12 h-12">
                  <img
                    alt="Rocketman - HTML Admin Template"
                    className="rounded-full"
                    src={$f()[0].images[1]}
                  />
                </div>
              </div>
              <div>
                <a href="" className="text-primary font-medium">
                  {$f()[0].users[0].name}
                </a>
                {$f()[0].news[0].shortContent}.
                <div className="text-slate-500 text-xs mt-1.5">
                  {$f()[0].dates[0]} - {$f()[0].times[0]}
                </div>
              </div>
            </div>
          </div>
          {/* END: Timeline Content Latest */}
          {/* BEGIN: Timeline Content */}
          <div className="lg:mr-[51%] pl-6 lg:pl-0 lg:pr-[51px] before:content-[''] before:absolute before:w-20 before:h-px before:mt-8 before:right-[60px] before:bg-slate-200 before:dark:bg-darkmode-400 before:rounded-full before:inset-x-0 before:mx-auto before:z-[-1]">
            <div className="bg-white dark:bg-darkmode-400 shadow-sm border border-slate-200 rounded-md p-5 flex flex-col sm:flex-row items-start gap-y-3 mt-10 before:content-[''] before:absolute before:w-5 before:h-5 before:bg-slate-200 before:rounded-full before:inset-x-0 before:ml-0.5 lg:before:ml-auto before:mr-auto before:dark:bg-darkmode-300 after:content-[''] after:absolute after:w-3 after:h-3 after:bg-slate-50 after:rounded-full after:inset-x-0 after:ml-1.5 lg:after:ml-auto after:mr-auto after:mt-1 after:dark:bg-darkmode-200">
              <div className="mr-3">
                <div className="image-fit w-12 h-12">
                  <img
                    alt="Rocketman - HTML Admin Template"
                    className="rounded-full"
                    src={$f()[0].images[2]}
                  />
                </div>
              </div>
              <div>
                <a href="" className="text-primary font-medium">
                  {$f()[1].users[0].name}
                </a>
                {$f()[1].news[0].shortContent}.
                <div className="text-slate-500 text-xs mt-1.5">
                  {$f()[1].dates[0]} - {$f()[1].times[0]}
                </div>
              </div>
            </div>
          </div>
          {/* END: Timeline Content */}
          <div className="relative z-10 bg-white dark:bg-darkmode-600 py-2 my-5 text-center text-slate-500 text-xs">
            April, 2022
          </div>
          {/* BEGIN: Timeline Content */}
          <div className="lg:ml-[51%] pl-6 lg:pl-[51px] before:content-[''] before:absolute before:w-20 before:h-px before:mt-8 before:left-[60px] before:bg-slate-200 before:dark:bg-darkmode-400 before:rounded-full before:inset-x-0 before:mx-auto before:z-[-1]">
            <div className="bg-white dark:bg-darkmode-400 shadow-sm border border-slate-200 rounded-md p-5 flex flex-col sm:flex-row items-start gap-y-3 mt-10 before:content-[''] before:absolute before:w-5 before:h-5 before:bg-slate-200 before:rounded-full before:inset-x-0 before:ml-0.5 lg:before:ml-auto before:mr-auto before:dark:bg-darkmode-300 after:content-[''] after:absolute after:w-3 after:h-3 after:bg-slate-50 after:rounded-full after:inset-x-0 after:ml-1.5 lg:after:ml-auto after:mr-auto after:mt-1 after:dark:bg-darkmode-200">
              <div className="mr-3">
                <div className="image-fit w-12 h-12">
                  <img
                    alt="Rocketman - HTML Admin Template"
                    className="rounded-full"
                    src={$f()[0].images[3]}
                  />
                </div>
              </div>
              <div>
                <a href="" className="text-primary font-medium">
                  {$f()[2].users[0].name}
                </a>
                {$f()[2].news[0].shortContent}.
                <div className="text-slate-500 text-xs mt-1.5">
                  {$f()[2].dates[0]} - {$f()[2].times[0]}
                </div>
                <div className="grid grid-cols-12 gap-2 mt-5">
                  <div className="image-fit col-span-6 sm:col-span-3 lg:col-span-2 h-16 cursor-pointer">
                    <img
                      alt="Rocketman - HTML Admin Template"
                      className="rounded-md"
                      src={$f()[0].images[0]}
                    />
                  </div>
                  <div className="image-fit col-span-6 sm:col-span-3 lg:col-span-2 h-16 cursor-pointer">
                    <img
                      alt="Rocketman - HTML Admin Template"
                      className="rounded-md"
                      src={$f()[0].images[1]}
                    />
                  </div>
                  <div className="image-fit col-span-6 sm:col-span-3 lg:col-span-2 h-16 cursor-pointer">
                    <img
                      alt="Rocketman - HTML Admin Template"
                      className="rounded-md"
                      src={$f()[0].images[2]}
                    />
                  </div>
                  <div className="image-fit col-span-6 sm:col-span-3 lg:col-span-2 h-16 cursor-pointer">
                    <img
                      alt="Rocketman - HTML Admin Template"
                      className="rounded-md"
                      src={$f()[0].images[3]}
                    />
                  </div>
                </div>
              </div>
            </div>
          </div>
          {/* END: Timeline Content */}
          {/* BEGIN: Timeline Content */}
          <div className="lg:ml-[51%] pl-6 lg:pl-[51px] before:content-[''] before:absolute before:w-20 before:h-px before:mt-8 before:left-[60px] before:bg-slate-200 before:dark:bg-darkmode-400 before:rounded-full before:inset-x-0 before:mx-auto before:z-[-1]">
            <div className="bg-white dark:bg-darkmode-400 shadow-sm border border-slate-200 rounded-md p-5 flex flex-col sm:flex-row items-start gap-y-3 mt-10 before:content-[''] before:absolute before:w-5 before:h-5 before:bg-slate-200 before:rounded-full before:inset-x-0 before:ml-0.5 lg:before:ml-auto before:mr-auto before:dark:bg-darkmode-300 after:content-[''] after:absolute after:w-3 after:h-3 after:bg-slate-50 after:rounded-full after:inset-x-0 after:ml-1.5 lg:after:ml-auto after:mr-auto after:mt-1 after:dark:bg-darkmode-200">
              <div className="mr-3">
                <div className="image-fit w-12 h-12">
                  <img
                    alt="Rocketman - HTML Admin Template"
                    className="rounded-full"
                    src={$f()[0].images[4]}
                  />
                </div>
              </div>
              <div>
                <a href="" className="text-primary font-medium">
                  {$f()[3].users[0].name}
                </a>
                {$f()[3].news[0].shortContent}.
                <div className="text-slate-500 text-xs mt-1.5">
                  {$f()[3].dates[0]} - {$f()[3].times[0]}
                </div>
              </div>
            </div>
          </div>
          {/* END: Timeline Content */}
          {/* BEGIN: Timeline Content */}
          <div className="lg:mr-[51%] pl-6 lg:pl-0 lg:pr-[51px] before:content-[''] before:absolute before:w-20 before:h-px before:mt-8 before:right-[60px] before:bg-slate-200 before:dark:bg-darkmode-400 before:rounded-full before:inset-x-0 before:mx-auto before:z-[-1]">
            <div className="bg-white dark:bg-darkmode-400 shadow-sm border border-slate-200 rounded-md p-5 flex flex-col sm:flex-row items-start gap-y-3 mt-10 before:content-[''] before:absolute before:w-5 before:h-5 before:bg-slate-200 before:rounded-full before:inset-x-0 before:ml-0.5 lg:before:ml-auto before:mr-auto before:dark:bg-darkmode-300 after:content-[''] after:absolute after:w-3 after:h-3 after:bg-slate-50 after:rounded-full after:inset-x-0 after:ml-1.5 lg:after:ml-auto after:mr-auto after:mt-1 after:dark:bg-darkmode-200">
              <div className="mr-3">
                <div className="image-fit w-12 h-12">
                  <img
                    alt="Rocketman - HTML Admin Template"
                    className="rounded-full"
                    src={$f()[0].images[5]}
                  />
                </div>
              </div>
              <div>
                <a href="" className="text-primary font-medium">
                  {$f()[4].users[0].name}
                </a>
                {$f()[4].news[0].shortContent}.
                <div className="text-slate-500 text-xs mt-1.5">
                  {$f()[4].dates[0]} - {$f()[4].times[0]}
                </div>
              </div>
            </div>
          </div>
          {/* END: Timeline Content */}
          {/* BEGIN: Timeline Content */}
          <div className="lg:mr-[51%] pl-6 lg:pl-0 lg:pr-[51px] before:content-[''] before:absolute before:w-20 before:h-px before:mt-8 before:right-[60px] before:bg-slate-200 before:dark:bg-darkmode-400 before:rounded-full before:inset-x-0 before:mx-auto before:z-[-1]">
            <div className="bg-white dark:bg-darkmode-400 shadow-sm border border-slate-200 rounded-md p-5 flex flex-col sm:flex-row items-start gap-y-3 mt-10 before:content-[''] before:absolute before:w-5 before:h-5 before:bg-slate-200 before:rounded-full before:inset-x-0 before:ml-0.5 lg:before:ml-auto before:mr-auto before:dark:bg-darkmode-300 after:content-[''] after:absolute after:w-3 after:h-3 after:bg-slate-50 after:rounded-full after:inset-x-0 after:ml-1.5 lg:after:ml-auto after:mr-auto after:mt-1 after:dark:bg-darkmode-200">
              <div className="mr-3">
                <div className="image-fit w-12 h-12">
                  <img
                    alt="Rocketman - HTML Admin Template"
                    className="rounded-full"
                    src={$f()[0].images[6]}
                  />
                </div>
              </div>
              <div>
                <a href="" className="text-primary font-medium">
                  {$f()[5].users[0].name}
                </a>
                {$f()[5].news[0].shortContent}.
                <div className="text-slate-500 text-xs mt-1.5">
                  {$f()[5].dates[0]} - {$f()[5].times[0]}
                </div>
              </div>
            </div>
          </div>
          {/* END: Timeline Content */}
          <button className="btn btn-outline-secondary relative z-10 bg-white dark:bg-darkmode-400 mt-10 mx-auto block">
            Load More
          </button>
        </div>
        {/* END: Timeline Wrapper */}
      </div>
    </>
  );
}

export default Main;
