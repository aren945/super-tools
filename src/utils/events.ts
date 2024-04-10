import events from "../../constants/events.json";

// 获取窗口隐藏事件名
export const getWinodwHiddenEvent = () => {
  return events["window_hide"];
};
