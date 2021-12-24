import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import * as FileSaver from 'file-saver';
import { Clipboard } from '@angular/cdk/clipboard';
import { MatSnackBar } from '@angular/material/snack-bar';
import { Friend, FriendService } from '../friend.service';
import { AddFriendComponent } from '../add-friend/add-friend.component';

@Component({
  selector: 'app-friends',
  templateUrl: './friends.component.html',
  styleUrls: ['./friends.component.scss'],
})
export class FriendsComponent implements OnInit {
  friends: Friend[] = [];

  constructor(
    private readonly friendService: FriendService,
    private readonly dialog: MatDialog,
    private readonly clipboard: Clipboard,
    private readonly snackBar: MatSnackBar
  ) {}

  ngOnInit(): void {
    this.refresh();
  }

  refresh() {
    this.friendService
      .getFriends()
      .subscribe((friends) => (this.friends = friends));
  }

  open_add_friend_dialog() {
    let dialogRef = this.dialog.open(AddFriendComponent, {
      panelClass: 'no-padded-dialog',
      disableClose: true,
    });
    dialogRef.afterClosed().subscribe(() => {
      this.refresh();
    });
  }

  exportPublicKeyToFile(friend: Friend) {
    let blob = new Blob([friend.publicKey]);
    FileSaver.saveAs(blob, `${friend.name}.pub.pem`);
  }

  exportPublicKeyToClipboard(friend: Friend) {
    this.clipboard.copy(friend.publicKey);
    this.snackBar.open('Public key copied', undefined, { duration: 1000 });
  }
}
