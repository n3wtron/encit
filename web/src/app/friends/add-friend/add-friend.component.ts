import { Component, EventEmitter, Output } from '@angular/core';
import { ErrorService } from '../../common/error.service';
import { MatDialogRef } from '@angular/material/dialog';
import { FriendService } from '../friend.service';

@Component({
  selector: 'app-new-identity',
  templateUrl: './add-friend.component.html',
  styleUrls: ['./add-friend.component.scss'],
})
export class AddFriendComponent {
  name: string = '';
  publicKey: string = '';
  keyFormat: 'PEM' | 'BASE64' | 'HEX' | undefined;

  @Output()
  newFriendAdded = new EventEmitter<void>();

  constructor(
    private readonly friendService: FriendService,
    private readonly errorService: ErrorService,
    private readonly dialogRef: MatDialogRef<AddFriendComponent>
  ) {}

  close(): void {
    this.dialogRef.close();
  }

  add() {
    if (!!this.name && !!this.keyFormat && !!this.publicKey) {
      this.friendService
        .add(this.name, this.keyFormat, this.publicKey)
        .subscribe(
          () => {
            this.newFriendAdded.emit();
            this.name = '';
            this.dialogRef.close();
          },
          (error) => this.errorService.error(error)
        );
    }
  }
}
