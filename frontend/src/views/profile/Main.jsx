import { Lucide } from "@/base-components";
 
import * as $_ from "lodash";
 

function Main() {
  return (
    <>
      <div className="intro-y flex flex-col sm:flex-row items-center mt-8">
        <h2 className="flex items-center text-lg font-medium mr-auto">
          Profile
        </h2>
        <div className="w-full sm:w-auto flex mt-4 sm:mt-0">
          <button className="btn btn-primary shadow-md mr-2">
            <Lucide icon="Pencil" className="w-4 h-4 mr-2" /> Update Profile
          </button>
          <button className="btn btn-outline-secondary shadow-md">
            <Lucide icon="Download" className="w-4 h-4 mr-2" /> View Profile
          </button>
        </div>
      </div>
      <div className="grid grid-cols-12 gap-5 mt-5">
        {/* BEGIN: Profile Cover */}
        <div className="col-span-12">
          <div className="box intro-y px-3 pt-3 pb-5">
            <div className="image-fit h-80 before:content-[''] before:absolute before:w-full before:h-full before:bg-gradient-to-b from-black/20 to-black before:rounded-md before:z-10">
               
            </div>
            <div className="flex flex-col 2xl:flex-row items-center justify-center text-center 2xl:text-left">
              <div className="-mt-20 2xl:-mt-10 2xl:ml-10 z-20">
                <div className="image-fit w-40 h-40 rounded-full border-4 border-white shadow-md overflow-hidden">
                 
                </div>
              </div>
              <div className="2xl:ml-5">
                <h2 className="text-2xl mt-5 font-medium">
                  {$f()[0].users[0].name}
                </h2>
                <div className="mt-2 text-slate-500 flex items-center justify-center 2xl:justify-start">
                  <Lucide icon="Briefcase" className="w-4 h-4 mr-2" /> Frontend
                  Engineer at Left4code Express
                </div>
                <div className="mt-2 text-slate-500 flex items-center justify-center 2xl:justify-start">
                  <Lucide icon="MapPin" className="w-4 h-4 mr-2" /> New York,
                  USA
                </div>
              </div>
              <div className="mx-auto grid grid-cols-2 gap-y-2 md:gap-y-0 gap-x-5 h-20 mt-5 2xl:border-l 2xl:border-r border-dashed border-slate-200 px-10 mb-6 2xl:mb-0">
                <div className="col-span-2 md:col-span-1 flex items-center justify-center 2xl:justify-start">
                  <Lucide icon="Mail" className="w-4 h-4 mr-2" />
                  johnnydepp@left4code.com
                </div>
                <div className="col-span-2 md:col-span-1 flex items-center justify-center 2xl:justify-start">
                  <Lucide icon="Instagram" className="w-4 h-4 mr-2" />{" "}
                  @johnnydepp
                </div>
                <div className="col-span-2 md:col-span-1 flex items-center justify-center 2xl:justify-start">
                  <Lucide icon="Twitter" className="w-4 h-4 mr-2" /> Johnny Depp
                </div>
                <div className="col-span-2 md:col-span-1 flex items-center justify-center 2xl:justify-start">
                  <Lucide icon="Linkedin" className="w-4 h-4 mr-2" /> Johnny
                  Depp
                </div>
              </div>
              <div className="flex 2xl:mr-10 mt-5">
                <button className="btn btn-primary mr-2 w-32">
                  <Lucide icon="UserPlus" className="w-4 h-4 mr-2" /> Following
                </button>
                <button className="btn btn-outline-secondary w-32">
                  <Lucide icon="UserCheck" className="w-4 h-4 mr-2" /> Add
                  Friend
                </button>
              </div>
            </div>
          </div>
        </div>
        {/* END: Profile Cover */}
        {/* BEGIN: Profile Content */}
        <div className="col-span-12 xl:col-span-8">
          <div className="box intro-y p-5">
            <div className="flex items-center border-b border-slate-200/60 dark:border-darkmode-400 pb-5 mb-5">
              <div className="font-medium truncate text-base">Profile</div>
              <Lucide icon="Edit" className="w-4 h-4 text-slate-500 ml-auto" />
            </div>
            <div className="leading-relaxed">
              <p className="mt-5">{$f()[0].news[0].content}</p>
              <p className="mt-5">{$f()[0].news[1].content}</p>
              <button className="btn btn-outline-secondary border-slate-200/60 w-full flex mt-5">
                <Lucide icon="ChevronDown" className="w-4 h-4 mr-2" /> View More
              </button>
            </div>
          </div>
          <div className="box intro-y p-5 mt-5">
            <div className="flex items-center border-b border-slate-200/60 dark:border-darkmode-400 pb-5 mb-5">
              <div className="font-medium truncate text-base">Experience</div>
              <Lucide icon="Edit" className="w-4 h-4 text-slate-500 ml-auto" />
            </div>
            <div>
              <div className="flex border-b border-slate-200 border-dashed pb-5 mb-5 last:border-b-0 last:pb-0 last:mb-0">
                <div className="mr-5">
                  <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                    SU
                  </div>
                </div>
                <div>
                  <div className="font-medium text-base">Left4code Express</div>
                  <div className="mt-1 text-slate-500">
                    Senior Frontend Engineer
                  </div>
                  <div className="mt-1">2005 - 2009 • 4 yrs</div>
                  <ul className="mt-5 sm:mt-3 list-disc -ml-16 sm:ml-3">
                    <li className="mb-1 last:mb-0">
                      Work across the full stack, building highly scalable
                      distributed solutions that enable positive user
                      experiences and measurable business growth.
                    </li>
                    <li className="mb-1 last:mb-0">
                      Develop new features and infrastructure development in
                      support of rapidly emerging business and project
                      requirements.
                    </li>
                    <li className="mb-1 last:mb-0">
                      Assume leadership of new projects from conceptualization
                      to deployment.
                    </li>
                    <li className="mb-1 last:mb-0">
                      Ensure application performance, uptime, and scale,
                      maintaining high standards of code quality and thoughtful
                      application design.
                    </li>
                    <li className="mb-1 last:mb-0">
                      Work with agile development methodologies, adhering to
                      best practices and pursuing continued learning
                      opportunities.
                    </li>
                  </ul>
                </div>
              </div>
              <div className="flex border-b border-slate-200 border-dashed pb-5 mb-5 last:border-b-0 last:pb-0 last:mb-0">
                <div className="mr-5">
                  <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                    UO
                  </div>
                </div>
                <div>
                  <div className="font-medium text-base">Freelancer</div>
                  <div className="mt-1 text-slate-500">Fullstack Engineer</div>
                  <div className="mt-1">2010 - 2014 • 4 yrs</div>
                  <ul className="mt-5 sm:mt-3 list-disc -ml-16 sm:ml-3">
                    <li className="mb-1 last:mb-0">
                      Participate in all aspects of agile software development
                      including design, implementation, and deployment
                    </li>
                    <li className="mb-1 last:mb-0">
                      Architect and provide guidance on building end-to-end
                      systems optimized for speed and scale
                    </li>
                    <li className="mb-1 last:mb-0">
                      Work primarily in Ruby, Java/JRuby, React, and JavaScript
                    </li>
                    <li className="mb-1 last:mb-0">
                      Engage with inspiring designers and front end engineers,
                      and collaborate with leading back end engineers as we
                      create reliable APIs
                    </li>
                    <li className="mb-1 last:mb-0">
                      Collaborate across time zones via Slack, GitHub comments,
                      documents, and frequent video conferences
                    </li>
                  </ul>
                </div>
              </div>
            </div>
            <button className="btn btn-outline-secondary border-slate-200/60 w-full flex mt-5">
              <Lucide icon="ChevronDown" className="w-4 h-4 mr-2" /> View More
            </button>
          </div>
          <div className="box intro-y p-5 mt-5">
            <div className="flex items-center border-b border-slate-200/60 dark:border-darkmode-400 pb-5 mb-5">
              <div className="font-medium truncate text-base">Skills</div>
              <Lucide icon="Edit" className="w-4 h-4 text-slate-500 ml-auto" />
            </div>
            <div className="flex flex-wrap">
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Ruby
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Java/JRuby
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                React
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                JavaScript
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Typescript
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Bootstrap 5
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                TailwindCSS 3
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Vuejs
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Ruby
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Java/JRuby
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                React
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                JavaScript
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Typescript
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Bootstrap 5
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                TailwindCSS 3
              </div>
              <div className="px-3 py-1 bg-primary/10 border border-primary/10 rounded-full mr-2 mb-2">
                Vuejs
              </div>
            </div>
          </div>
          <div className="box intro-y p-5 mt-5">
            <div className="flex items-center border-b border-slate-200/60 dark:border-darkmode-400 pb-5 mb-5">
              <div className="font-medium truncate text-base">Interests</div>
              <Lucide icon="Edit" className="w-4 h-4 text-slate-500 ml-auto" />
            </div>
            <div className="grid grid-cols-12 gap-y-7">
              <div className="col-span-12 sm:col-span-6 2xl:col-span-4 flex">
                <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                  SV
                </div>
                <div className="ml-5">
                  <div className="font-medium text-base">Svelte</div>
                  <div className="mt-1 text-slate-500">4,468,655 followers</div>
                  <button className="btn btn-outline-secondary btn-rounded py-1 px-3 mt-2">
                    <Lucide icon="Plus" className="w-4 h-4 mr-1" /> Follow
                  </button>
                </div>
              </div>
              <div className="col-span-12 sm:col-span-6 2xl:col-span-4 flex">
                <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                  AN
                </div>
                <div className="ml-5">
                  <div className="font-medium text-base">Angular</div>
                  <div className="mt-1 text-slate-500">1,468,655 followers</div>
                  <button className="btn btn-outline-secondary btn-rounded py-1 px-3 mt-2">
                    <Lucide icon="Plus" className="w-4 h-4 mr-1" /> Follow
                  </button>
                </div>
              </div>
              <div className="col-span-12 sm:col-span-6 2xl:col-span-4 flex">
                <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                  TW
                </div>
                <div className="ml-5">
                  <div className="font-medium text-base">TailwindCSS</div>
                  <div className="mt-1 text-slate-500">
                    45,468,655 followers
                  </div>
                  <button className="btn btn-outline-secondary btn-rounded py-1 px-3 mt-2">
                    <Lucide icon="Plus" className="w-4 h-4 mr-1" /> Follow
                  </button>
                </div>
              </div>
              <div className="col-span-12 sm:col-span-6 2xl:col-span-4 flex">
                <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                  LV
                </div>
                <div className="ml-5">
                  <div className="font-medium text-base">Laravel</div>
                  <div className="mt-1 text-slate-500">4,468,655 followers</div>
                  <button className="btn btn-outline-secondary btn-rounded py-1 px-3 mt-2">
                    <Lucide icon="Plus" className="w-4 h-4 mr-1" /> Follow
                  </button>
                </div>
              </div>
              <div className="col-span-12 sm:col-span-6 2xl:col-span-4 flex">
                <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                  RT
                </div>
                <div className="ml-5">
                  <div className="font-medium text-base">React</div>
                  <div className="mt-1 text-slate-500">1,468,655 followers</div>
                  <button className="btn btn-outline-secondary btn-rounded py-1 px-3 mt-2">
                    <Lucide icon="Plus" className="w-4 h-4 mr-1" /> Follow
                  </button>
                </div>
              </div>
              <div className="col-span-12 sm:col-span-6 2xl:col-span-4 flex">
                <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                  BS
                </div>
                <div className="ml-5">
                  <div className="font-medium text-base">Bootstrap</div>
                  <div className="mt-1 text-slate-500">
                    45,468,655 followers
                  </div>
                  <button className="btn btn-outline-secondary btn-rounded py-1 px-3 mt-2">
                    <Lucide icon="Plus" className="w-4 h-4 mr-1" /> Follow
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
        {/* END: Profile Content */}
        {/* BEGIN: Profile Side Menu */}
        <div className="col-span-12 xl:col-span-4">
          <div className="box intro-y p-5">
            <div className="flex items-center border-b border-slate-200/60 dark:border-darkmode-400 pb-5 mb-5">
              <div className="font-medium truncate text-base">Education</div>
              <Lucide icon="Edit" className="w-4 h-4 text-slate-500 ml-auto" />
            </div>
            <div>
              <div className="flex border-b border-slate-200 border-dashed pb-5 mb-5 last:border-b-0 last:pb-0 last:mb-0">
                <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                  SU
                </div>
                <div className="ml-5">
                  <div className="font-medium text-base">
                    Stanford University
                  </div>
                  <div className="mt-1 text-slate-500">
                    Computer Science and Engineering
                  </div>
                  <div className="mt-1">2005 - 2009 • 4 yrs</div>
                  <div className="mt-1">California, USA</div>
                </div>
              </div>
              <div className="flex border-b border-slate-200 border-dashed pb-5 mb-5 last:border-b-0 last:pb-0 last:mb-0">
                <div className="w-16 h-16 rounded-full bg-slate-200 dark:bg-darkmode-400 flex items-center justify-center text-base font-medium">
                  UO
                </div>
                <div className="ml-5">
                  <div className="font-medium text-base">
                    University of Oxford
                  </div>
                  <div className="mt-1 text-slate-500">
                    Computer Science and Engineering
                  </div>
                  <div className="mt-1">2010 - 2014 • 4 yrs</div>
                  <div className="mt-1">Oxford, England</div>
                </div>
              </div>
            </div>
            <button className="btn btn-outline-secondary border-slate-200/60 w-full flex mt-5">
              <Lucide icon="ChevronDown" className="w-4 h-4 mr-2" /> View More
            </button>
          </div>
          <div className="box intro-y p-5 mt-5">
            <div className="flex items-center border-b border-slate-200/60 dark:border-darkmode-400 pb-5 mb-5">
              <div className="font-medium truncate text-base">
                Followers (102)
              </div>
            </div>
            <div>
              {$_.take($f(), 5).map((faker, fakerKey) => (
                <div
                  key={fakerKey}
                  className="flex items-center border-b border-slate-200 border-dashed pb-5 mb-5 last:border-b-0 last:pb-0 last:mb-0"
                >
                  <div>
                    <div className="w-16 h-16 image-fit">
                      <img
                        alt="Rocketman - HTML Admin Template"
                        className="rounded-full"
                        src={faker.photos[0]}
                      />
                    </div>
                  </div>
                  <div className="w-full ml-5 flex 2xl:items-center gap-y-3 flex-col 2xl:flex-row">
                    <div className="mr-auto">
                      <div className="font-medium text-base flex items-center">
                        <div className="whitespace-nowrap">
                          {faker.users[0].name}
                        </div>
                        <div className="mx-1.5">•</div>
                        <a href="" className="text-success text-xs">
                          Follow
                        </a>
                      </div>
                      <div className="mt-1 text-slate-500">
                        {faker.users[0].username}
                      </div>
                    </div>
                    <div className="flex">
                      <button className="btn btn-outline-secondary py-1 px-2">
                        <Lucide icon="UserCheck" className="w-4 h-4 mr-2" />{" "}
                        Friends
                      </button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
            <button className="btn btn-outline-secondary border-slate-200/60 w-full flex mt-5">
              <Lucide icon="ChevronDown" className="w-4 h-4 mr-2" /> View More
            </button>
          </div>
        </div>
        {/* END: Profile Side Menu */}
      </div>
    </>
  );
}

export default Main;
