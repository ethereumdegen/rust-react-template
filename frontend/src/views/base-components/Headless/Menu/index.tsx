import { twMerge } from "tailwind-merge";
import { Menu as HeadlessMenu, Transition } from "@headlessui/react";
import { Fragment } from "react";
import clsx from "clsx";

function Menu({
  children,
  className,
  ...props
}: ExtractProps<typeof HeadlessMenu>) {
  return (
    <HeadlessMenu
      as="div"
      className={twMerge(["relative", className])}
      {...props}
    >
      {children}
    </HeadlessMenu>
  );
}

Menu.Button = ({
  children,
  className,
  ...props
}: ExtractProps<typeof HeadlessMenu.Button>) => {
  return (
    <HeadlessMenu.Button
      as="div"
      className={twMerge(["cursor-pointer", className])}
      {...props}
    >
      {children}
    </HeadlessMenu.Button>
  );
};

Menu.Items = ({
  children,
  className,
  placement = "bottom-end",
  ...props
}: ExtractProps<typeof HeadlessMenu.Items> & {
  placement?:
    | "top-start"
    | "top"
    | "top-end"
    | "right-start"
    | "right"
    | "right-end"
    | "bottom-end"
    | "bottom"
    | "bottom-start"
    | "left-start"
    | "left"
    | "left-end";
}) => {
  return (
    <Transition
      as={Fragment}
      enter="transition-all ease-linear duration-150"
      enterFrom="mt-5 invisible opacity-0 translate-y-1"
      enterTo="mt-1 visible opacity-100 translate-y-0"
      leave="transition-all ease-linear duration-150"
      leaveFrom="mt-1 visible opacity-100 translate-y-0"
      leaveTo="mt-5 invisible opacity-0 translate-y-1"
    >
      <div
        className={clsx([
          "absolute z-30",
          placement == "top-start" && "left-0 bottom-[100%]",
          placement == "top" && "left-[50%] translate-x-[-50%] bottom-[100%]",
          placement == "top-end" && "right-0 bottom-[100%]",
          placement == "right-start" && "left-[100%] translate-y-[-50%]",
          placement == "right" && "left-[100%] top-[50%] translate-y-[-50%]",
          placement == "right-end" && "left-[100%] bottom-0",
          placement == "bottom-end" && "top-[100%] right-0",
          placement == "bottom" && "top-[100%] left-[50%] translate-x-[-50%]",
          placement == "bottom-start" && "top-[100%] left-0",
          placement == "left-start" && "right-[100%] translate-y-[-50%]",
          placement == "left" && "right-[100%] top-[50%] translate-y-[-50%]",
          placement == "left-end" && "right-[100%] bottom-0",
        ])}
      >
        <HeadlessMenu.Items
          as="div"
          className={twMerge([
            "p-2 shadow-[0px_3px_5px_#0000000b] bg-white border border-slate-200 rounded-lg dark:bg-darkmode-600 dark:border-darkmode-500",
            className,
          ])}
          {...props}
        >
          {children}
        </HeadlessMenu.Items>
      </div>
    </Transition>
  );
};

Menu.Item = ({
  children,
  className,
  ...props
}: ExtractProps<typeof HeadlessMenu.Item>) => {
  return (
    <HeadlessMenu.Item
      as="a"
      className={twMerge([
        "cursor-pointer flex items-center p-2 transition duration-300 ease-in-out rounded-md hover:bg-slate-200/60 dark:bg-darkmode-600 dark:hover:bg-darkmode-400",
        className,
      ])}
      {...props}
    >
      {children}
    </HeadlessMenu.Item>
  );
};

Menu.Divider = (props: React.ComponentPropsWithoutRef<"div">) => {
  return (
    <div
      className={twMerge([
        "h-px my-2 -mx-2 bg-slate-200/60 dark:bg-darkmode-400",
        props.className,
      ])}
    ></div>
  );
};

Menu.Header = (
  props: React.PropsWithChildren & React.ComponentPropsWithoutRef<"div">
) => {
  return (
    <div className={twMerge(["p-2 font-medium", props.className])}>
      {props.children}
    </div>
  );
};

Menu.Footer = (
  props: React.PropsWithChildren & React.ComponentPropsWithoutRef<"div">
) => {
  return (
    <div className={twMerge(["flex p-1", props.className])}>
      {props.children}
    </div>
  );
};

export default Menu;
