type EventID = string;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type EventHandler = (args: any) => void;

class EventBus {
  private events: Map<EventID, Array<EventHandler>>;

  public constructor() {
    this.events = new Map();
  }

  public addEventHandler(event: EventID, subscriber: EventHandler) {
    let eventHandlers = this.events.get(event);

    if (eventHandlers === undefined) {
      eventHandlers = [];

      this.events.set(event, eventHandlers);
    } else if (eventHandlers.indexOf(subscriber) === -1) return;

    eventHandlers.push(subscriber);
  }

  public removeEventHandler(event: EventID, subscriber: EventHandler) {
    const eventHandlers = this.events.get(event);

    if (eventHandlers) {
      const eventIndex = eventHandlers.indexOf(subscriber);

      if (eventIndex !== -1) {
        eventHandlers.splice(eventIndex, 1);
      }
    }
  }

  public invoke(event: EventID, args: object) {
    const eventHandlers = this.events.get(event);

    if (eventHandlers) {
      eventHandlers.forEach((eventHandler) => eventHandler(args));
    }
  }
}

const eventBus = new EventBus();

export { EventBus, EventID as EventIdentifier, EventHandler, eventBus };
