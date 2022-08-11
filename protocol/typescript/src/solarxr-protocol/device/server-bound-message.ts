// automatically generated by the FlatBuffers compiler, do not modify

import { IChooseYouRequest, IChooseYouRequestT } from '../../solarxr-protocol/device/commands/ichoose-you-request';
import { SetWifiRequest, SetWifiRequestT } from '../../solarxr-protocol/device/commands/set-wifi-request';
import { PollDataFeedRequest, PollDataFeedRequestT } from '../../solarxr-protocol/device/data-feed/poll-data-feed-request';
import { StartDataFeedRequest, StartDataFeedRequestT } from '../../solarxr-protocol/device/data-feed/start-data-feed-request';


export enum ServerBoundMessage{
  NONE = 0,
  solarxr_protocol_device_data_feed_StartDataFeedRequest = 1,
  solarxr_protocol_device_data_feed_PollDataFeedRequest = 2,
  solarxr_protocol_device_commands_SetWifiRequest = 3,
  solarxr_protocol_device_commands_IChooseYouRequest = 4
}

export function unionToServerBoundMessage(
  type: ServerBoundMessage,
  accessor: (obj:IChooseYouRequest|PollDataFeedRequest|SetWifiRequest|StartDataFeedRequest) => IChooseYouRequest|PollDataFeedRequest|SetWifiRequest|StartDataFeedRequest|null
): IChooseYouRequest|PollDataFeedRequest|SetWifiRequest|StartDataFeedRequest|null {
  switch(ServerBoundMessage[type]) {
    case 'NONE': return null; 
    case 'solarxr_protocol_device_data_feed_StartDataFeedRequest': return accessor(new StartDataFeedRequest())! as StartDataFeedRequest;
    case 'solarxr_protocol_device_data_feed_PollDataFeedRequest': return accessor(new PollDataFeedRequest())! as PollDataFeedRequest;
    case 'solarxr_protocol_device_commands_SetWifiRequest': return accessor(new SetWifiRequest())! as SetWifiRequest;
    case 'solarxr_protocol_device_commands_IChooseYouRequest': return accessor(new IChooseYouRequest())! as IChooseYouRequest;
    default: return null;
  }
}

export function unionListToServerBoundMessage(
  type: ServerBoundMessage, 
  accessor: (index: number, obj:IChooseYouRequest|PollDataFeedRequest|SetWifiRequest|StartDataFeedRequest) => IChooseYouRequest|PollDataFeedRequest|SetWifiRequest|StartDataFeedRequest|null, 
  index: number
): IChooseYouRequest|PollDataFeedRequest|SetWifiRequest|StartDataFeedRequest|null {
  switch(ServerBoundMessage[type]) {
    case 'NONE': return null; 
    case 'solarxr_protocol_device_data_feed_StartDataFeedRequest': return accessor(index, new StartDataFeedRequest())! as StartDataFeedRequest;
    case 'solarxr_protocol_device_data_feed_PollDataFeedRequest': return accessor(index, new PollDataFeedRequest())! as PollDataFeedRequest;
    case 'solarxr_protocol_device_commands_SetWifiRequest': return accessor(index, new SetWifiRequest())! as SetWifiRequest;
    case 'solarxr_protocol_device_commands_IChooseYouRequest': return accessor(index, new IChooseYouRequest())! as IChooseYouRequest;
    default: return null;
  }
}

