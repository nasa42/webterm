export const alertAndThrow = (message: string): never => {
  alert(message);
  throw new Error(message);
};
