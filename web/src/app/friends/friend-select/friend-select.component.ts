import {Component, EventEmitter, Input, OnInit, Output} from '@angular/core';
import {Friend, FriendService} from "../friend.service";

@Component({
  selector: 'app-friend-select',
  templateUrl: './friend-select.component.html',
  styleUrls: ['./friend-select.component.scss']
})
export class FriendSelectComponent implements OnInit {
  @Input()
  friend: Friend | undefined;

  @Output()
  friendChange = new EventEmitter<Friend | undefined>();

  setModel(selFriend: Friend | undefined) {
    this.friend = selFriend;
    this.friendChange.emit(selFriend);
  }

  friends: Friend[] = [];

  constructor(private readonly friendService: FriendService) {
  }

  ngOnInit(): void {
    this.friendService.getFriends().subscribe(friends => this.friends = friends);
  }

}
