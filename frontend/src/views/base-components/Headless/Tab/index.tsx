import { twMerge } from "tailwind-merge";
import { Tab as HeadlessTab, Transition } from "@headlessui/react";
import { Fragment, createContext, useContext } from "react";

type Variant = "tabs" | "pills" | "boxed-tabs" | "link-tabs";

const tabContext = createContext<{
  selected: boolean;
}>({
  selected: false,
});

const listContext = createContext<{
  variant: Variant;
}>({
  variant: "tabs",
});

function Tab({
  children,
  className,
  fullWidth = true,
  ...props
}: ExtractProps<typeof HeadlessTab> & {
  fullWidth?: boolean;
}) {
  const list = useContext(listContext);
  return (
    <HeadlessTab as={Fragment}>
      {({ selected }) => (
        <li
          className={twMerge([
            "focus-visible:outline-none",
            fullWidth && "flex-1",
            list.variant == "tabs" && "-mb-px",
          ])}
          {...props}
        >
          <tabContext.Provider
            value={{
              selected: selected,
            }}
          >
            {typeof children === "function"
              ? children({
                  selected: selected,
                })
              : children}
          </tabContext.Provider>
        </li>
      )}
    </HeadlessTab>
  );
}

Tab.Button = <C extends React.ElementType = "a">({
  children,
  className,
  as,
  clicked,
  ...props
}: {
  as?: C;
} & React.PropsWithChildren &
  React.ComponentPropsWithoutRef<C>) => {
  const tab = useContext(tabContext);
  const list = useContext(listContext);
  const Component = as || "a";

  return (
    <Component
      onClick={clicked}
      className={twMerge([
        "cursor-pointer block appearance-none px-5 py-2.5 border border-transparent text-slate-700 dark:text-slate-400",
        tab.selected && "text-slate-800 dark:text-white",

        // Default
        list.variant == "tabs" &&
          "block border-transparent rounded-t-md dark:border-transparent",
        list.variant == "tabs" &&
          tab.selected &&
          "bg-white border-slate-200 border-b-transparent font-medium dark:bg-transparent dark:border-t-darkmode-400 dark:border-b-darkmode-600 dark:border-x-darkmode-400",
        list.variant == "tabs" &&
          !tab.selected &&
          "hover:bg-slate-100 dark:hover:bg-darkmode-400 dark:hover:border-transparent",

        // Pills
        list.variant == "pills" && "rounded-md border-0",
        list.variant == "pills" &&
          tab.selected &&
          "bg-primary text-white font-medium",

        // Boxed tabs
        list.variant == "boxed-tabs" &&
          "rounded-md py-1 dark:border-transparent",
        list.variant == "boxed-tabs" &&
          tab.selected &&
          "text-slate-700 border font-medium border-slate-200 bg-slate-50 dark:text-slate-300 dark:bg-darkmode-400 dark:border-darkmode-400",

        // Link tabs
        list.variant == "link-tabs" &&
          "border-b-2 border-transparent dark:border-transparent",
        list.variant == "link-tabs" &&
          tab.selected &&
          "border-b-primary font-medium dark:border-b-primary",

        className,
      ])}
      {...props}
    >
      {children}
    </Component>
  );
};

Tab.Group = ({
  children,
  ...props
}: ExtractProps<typeof HeadlessTab.Group>) => {
  return (
    <HeadlessTab.Group as="div" {...props}>
      {children}
    </HeadlessTab.Group>
  );
};

Tab.List = ({
  children,
  className,
  variant = "tabs",
  ...props
}: ExtractProps<typeof HeadlessTab.List> & {
  variant?: Variant;
}) => {
  return (
    <listContext.Provider
      value={{
        variant: variant,
      }}
    >
      <HeadlessTab.List
        as="ul"
        className={twMerge([
          variant == "tabs" &&
            "border-b border-slate-200 dark:border-darkmode-400",
          variant == "boxed-tabs" &&
            "p-1 border border-slate-200 rounded-lg dark:border-darkmode-400",
          "w-full flex",
          className,
        ])}
        {...props}
      >
        {children}
      </HeadlessTab.List>
    </listContext.Provider>
  );
};

Tab.Panels = ({
  children,
  className,
  ...props
}: ExtractProps<typeof HeadlessTab.Panels>) => {
  return (
    <HeadlessTab.Panels as="div" className={className} {...props}>
      {children}
    </HeadlessTab.Panels>
  );
};

Tab.Panel = ({
  children,
  className,
  ...props
}: ExtractProps<typeof HeadlessTab.Panel>) => {
  return (
    <HeadlessTab.Panel as={Fragment}>
      {({ selected }) => (
        <Transition
          appear
          as="div"
          show={selected}
          enter="transition-opacity duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="transition-opacity duration-300"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
          className={className}
          {...props}
        >
          <>
            {typeof children === "function"
              ? children({
                  selected: selected,
                })
              : children}
          </>
        </Transition>
      )}
    </HeadlessTab.Panel>
  );
};

export default Tab;
